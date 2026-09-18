use super::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
pub(crate) fn identifier(s: &str) -> bool {
    (1..=64).contains(&s.len())
        && s.as_bytes()[0].is_ascii_alphabetic()
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
}
pub(crate) fn raw_model(v: &Value) -> Result<()> {
    // Option<T> intentionally omits absent fields when serialized; explicit null is not absence.
    fn walk(v: &Value) -> Result<()> {
        match v {
            Value::Null => {
                return Err(BoxUiError::new(
                    "source-shape",
                    "Null is not allowed in a model",
                ));
            }
            Value::Array(a) => {
                for v in a {
                    walk(v)?;
                }
            }
            Value::Object(m) => {
                for v in m.values() {
                    walk(v)?;
                }
            }
            _ => (),
        }
        Ok(())
    }
    walk(v)
}
pub fn validate_boxui(doc: &BoxUiDocument) -> Result<()> {
    if doc.profile != "boxui/0.1" {
        return Err(BoxUiError::new("unsupported-profile", "Expected boxui/0.1"));
    }
    if !identifier(&doc.document_id) {
        return Err(BoxUiError::new("invalid-id", "Invalid documentId"));
    }
    if doc.bindings.len() > 256 {
        return Err(BoxUiError::new("binding-budget", "At most 256 bindings"));
    }
    if !doc.root.kind.is_region() {
        return Err(BoxUiError::new("root-kind", "Root must be row or column"));
    }
    let mut bindings = BTreeMap::new();
    for b in &doc.bindings {
        if !identifier(&b.id) || bindings.insert(b.id.as_str(), b).is_some() {
            return Err(BoxUiError::new(
                "binding-id",
                "Invalid or duplicate binding ID",
            ));
        }
        if b.role == BindingRole::Value && b.data_type == DataType::None {
            return Err(BoxUiError::new(
                "binding-type",
                "Value cannot have type none",
            ));
        }
    }
    let mut ids = BTreeSet::new();
    let mut diagrams = 0;
    fn visit<'a>(
        n: &'a Node,
        depth: usize,
        ids: &mut BTreeSet<&'a str>,
        diagrams: &mut usize,
        bindings: &BTreeMap<&str, &Binding>,
    ) -> Result<()> {
        let err = |code: &str, msg: &str| BoxUiError::new(code, msg).node(&n.id);
        if depth > 16 || ids.len() >= 256 {
            return Err(err("node-budget", "At most 256 nodes and depth 16"));
        }
        if !identifier(&n.id) || !ids.insert(&n.id) {
            return Err(err("node-id", "Invalid or duplicate node ID"));
        }
        if !WidgetRegistry::supports(n.kind, n.version) {
            return Err(err("widget-version", "Unsupported widget version"));
        }
        let allowed: &[&str] = match n.kind {
            Kind::Row | Kind::Column => &["children"],
            Kind::Text => &["text"],
            Kind::Value => &["label", "valueBinding"],
            Kind::Button => &["label", "commandBinding"],
            Kind::Input => &["label", "valueBinding", "commandBinding"],
            Kind::Diagram => &["label", "family", "childRef"],
        };
        for (field, present) in [
            ("children", n.children.is_some()),
            ("text", n.text.is_some()),
            ("label", n.label.is_some()),
            ("valueBinding", n.value_binding.is_some()),
            ("commandBinding", n.command_binding.is_some()),
            ("family", n.family.is_some()),
            ("childRef", n.child_ref.is_some()),
        ] {
            if present != allowed.contains(&field) {
                return Err(err(
                    "widget-fields",
                    &format!("{field} is missing or forbidden for {:?}", n.kind),
                ));
            }
        }
        if n.label
            .iter()
            .chain(n.text.iter())
            .any(|s| s.chars().count() > 4096)
        {
            return Err(err("text-budget", "Text exceeds 4096 characters"));
        }
        if let Some(s) = &n.size {
            for x in [s.min, s.max].into_iter().flatten() {
                if !x.is_finite() || !(0.0..=8192.0).contains(&x) {
                    return Err(err("invalid-size", "Size outside 0..8192"));
                }
            }
            if s.grow
                .is_some_and(|g| !g.is_finite() || !(0.0..=1000.0).contains(&g))
                || matches!((s.min,s.max),(Some(a),Some(b)) if a>b)
            {
                return Err(err("invalid-size", "Invalid grow or min > max"));
            }
        }
        for (id, role) in [
            (n.value_binding.as_deref(), BindingRole::Value),
            (n.command_binding.as_deref(), BindingRole::Command),
        ] {
            if let Some(id) = id {
                let b = bindings
                    .get(id)
                    .ok_or_else(|| err("binding-missing", &format!("Unknown binding {id}")))?;
                if b.role != role
                    || (n.kind == Kind::Input && b.data_type != DataType::String)
                    || (n.kind == Kind::Button && b.data_type != DataType::None)
                {
                    return Err(err("binding-type", &format!("Incompatible binding {id}")));
                }
            }
        }
        if n.kind == Kind::Diagram {
            *diagrams += 1;
            if *diagrams > 8 {
                return Err(err("child-budget", "At most 8 diagram panes"));
            }
            if n.child_ref.as_deref() != Some(n.id.as_str()) {
                return Err(err("child-ref", "childRef must equal node ID"));
            }
            if !matches!(
                n.family.as_deref(),
                Some("flowchart" | "sequenceDiagram" | "stateDiagram-v2")
            ) {
                return Err(err("child-family", "Unsupported diagram family"));
            }
        }
        if n.kind.is_region() && n.children().is_empty() {
            return Err(err("empty-region", "Region requires children"));
        }
        for child in n.children() {
            visit(child, depth + 1, ids, diagrams, bindings)?;
        }
        Ok(())
    }
    visit(&doc.root, 1, &mut ids, &mut diagrams, &bindings)
}
