use super::model::*;
use anyhow::{Result, bail, ensure};
use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::Value;
use std::{collections::HashSet, fmt};
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildSource {
    pub r#ref: String,
    pub family: String,
    pub source: String,
    pub source_start: usize,
    pub source_end: usize,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseOutput {
    pub model: Document,
    pub child_sources: Vec<ChildSource>,
}
struct Unique(Value);
impl<'de> Deserialize<'de> for Unique {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Unique;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "strict JSON")
            }
            fn visit_map<M: MapAccess<'de>>(
                self,
                mut m: M,
            ) -> std::result::Result<Unique, M::Error> {
                let mut out = serde_json::Map::new();
                while let Some((k, v)) = m.next_entry::<String, Unique>()? {
                    if out.insert(k, v.0).is_some() {
                        return Err(de::Error::custom("duplicate JSON key"));
                    }
                }
                Ok(Unique(Value::Object(out)))
            }
            fn visit_seq<S: SeqAccess<'de>>(
                self,
                mut s: S,
            ) -> std::result::Result<Unique, S::Error> {
                let mut out = Vec::new();
                while let Some(v) = s.next_element::<Unique>()? {
                    out.push(v.0);
                }
                Ok(Unique(Value::Array(out)))
            }
            fn visit_str<E: de::Error>(self, s: &str) -> std::result::Result<Unique, E> {
                Ok(Unique(s.into()))
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_f64<E: de::Error>(self, v: f64) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_unit<E: de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::Null))
            }
        }
        d.deserialize_any(V)
    }
}
pub fn parse(source: &str) -> Result<ParseOutput> {
    ensure!(source.len() <= 262144, "BoxUI source exceeds 256 KiB");
    let (header, body) = source
        .trim_start()
        .split_once('\n')
        .ok_or_else(|| anyhow::anyhow!("Missing BoxUI body"))?;
    ensure!(
        header.trim() == "boxui 0.1",
        "Unsupported BoxUI header/version"
    );
    let mut raw: Value = serde_json::from_str::<Unique>(body)?.0;
    let mut child_sources = Vec::new();
    fn extract(n: &mut Value, out: &mut Vec<ChildSource>, depth: usize) -> Result<()> {
        ensure!(depth <= 16, "BoxUI depth exceeds 16");
        if n["kind"] == "diagram" {
            let id = n["id"].as_str().unwrap_or("").to_string();
            let family = n["family"].as_str().unwrap_or("").to_string();
            ensure!(
                n.get("childRef").is_none(),
                "childRef is not authoring syntax"
            );
            let source = n
                .as_object_mut()
                .and_then(|n| n.remove("source"))
                .and_then(|v| v.as_str().map(str::to_owned))
                .ok_or_else(|| anyhow::anyhow!("Diagram source required"))?;
            ensure!(source.len() <= 65536, "Child source exceeds 64 KiB");
            ensure!(
                source.split_whitespace().next() == Some(family.as_str()),
                "Child family/header mismatch"
            );
            n["childRef"] = id.clone().into();
            out.push(ChildSource {
                r#ref: id,
                family,
                source,
                source_start: 0,
                source_end: 0,
            });
            ensure!(out.len() <= 8, "At most 8 child diagrams");
        }
        if let Some(children) = n.get_mut("children").and_then(Value::as_array_mut) {
            for c in children {
                extract(c, out, depth + 1)?;
            }
        }
        Ok(())
    }
    extract(&mut raw["root"], &mut child_sources, 1)?;
    // Locate complete JSON string tokens, retaining byte offsets in the original fence.
    let sources = regex::Regex::new(r#""source"\s*:\s*("(?:[^"\\]|\\.)*")"#)?;
    let offset = source.len() - body.len();
    let mut tokens = sources.captures_iter(body);
    for child in &mut child_sources {
        let token = tokens
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing child span"))?;
        let token = token.get(1).unwrap();
        ensure!(
            serde_json::from_str::<String>(token.as_str())? == child.source,
            "Child span mismatch"
        );
        child.source_start = offset + token.start();
        child.source_end = offset + token.end();
    }
    let model: Document = serde_json::from_value(raw)?;
    validate(&model)?;
    Ok(ParseOutput {
        model,
        child_sources,
    })
}
pub fn validate(doc: &Document) -> Result<()> {
    ensure!(
        doc.profile == "boxui/0.1" && identifier(&doc.document_id),
        "Invalid BoxUI profile/documentId"
    );
    ensure!(doc.bindings.len() <= 256, "Too many bindings");
    let mut bindings = HashSet::new();
    for b in &doc.bindings {
        ensure!(
            identifier(&b.id) && bindings.insert(&b.id),
            "Duplicate/invalid binding ID"
        );
        ensure!(
            ["value", "command"].contains(&b.role.as_str()),
            "Invalid binding role"
        );
        ensure!(
            ["string", "number", "boolean", "none"].contains(&b.r#type.as_str()),
            "Invalid binding type"
        );
        ensure!(
            b.role != "value" || b.r#type != "none",
            "Value cannot have type none"
        );
    }
    let mut ids = HashSet::new();
    let mut children = 0;
    fn visit<'a>(
        n: &'a Node,
        doc: &Document,
        ids: &mut HashSet<&'a str>,
        count: &mut usize,
        depth: usize,
    ) -> Result<()> {
        ensure!(
            depth <= 16 && ids.len() < 256,
            "BoxUI node/depth budget exceeded"
        );
        ensure!(
            identifier(&n.id) && ids.insert(&n.id),
            "Duplicate/invalid node ID: {}",
            n.id
        );
        ensure!(n.version == 1, "Unsupported widget version: {}", n.id);
        for x in [n.size.min, n.size.max, n.size.grow].into_iter().flatten() {
            ensure!(
                x.is_finite() && (0.0..=8192.0).contains(&x),
                "Invalid size: {}",
                n.id
            );
        }
        ensure!(n.size.grow.unwrap_or(0.) <= 1000., "grow exceeds 1000");
        ensure!(
            n.size.min.unwrap_or(0.) <= n.size.max.unwrap_or(8192.),
            "min > max: {}",
            n.id
        );
        for t in [&n.label, &n.text].into_iter().flatten() {
            ensure!(t.chars().count() <= 4096, "Text exceeds limit");
        }
        let region = matches!(n.kind, Kind::Row | Kind::Column);
        ensure!(
            region != n.children.is_empty(),
            "Region requires children; leaf forbids children: {}",
            n.id
        );
        ensure!(
            n.text.is_some() == (n.kind == Kind::Text),
            "text property mismatch: {}",
            n.id
        );
        ensure!(
            n.label.is_some()
                == matches!(
                    n.kind,
                    Kind::Value | Kind::Button | Kind::Input | Kind::Diagram
                ),
            "label required/forbidden: {}",
            n.id
        );
        ensure!(
            n.value_binding.is_some() == matches!(n.kind, Kind::Value | Kind::Input),
            "valueBinding mismatch"
        );
        ensure!(
            n.command_binding.is_some() == matches!(n.kind, Kind::Button | Kind::Input),
            "commandBinding mismatch"
        );
        for (id, role) in [(&n.value_binding, "value"), (&n.command_binding, "command")] {
            if let Some(id) = id {
                let b = doc
                    .bindings
                    .iter()
                    .find(|b| b.id == *id && b.role == role)
                    .ok_or_else(|| anyhow::anyhow!("Unknown binding/role: {id}"))?;
                ensure!(
                    n.kind != Kind::Input || b.r#type == "string",
                    "Input requires string bindings"
                );
                ensure!(
                    n.kind != Kind::Button || b.r#type == "none",
                    "Button requires none command"
                );
            }
        }
        ensure!(
            n.child_ref.is_some() == (n.kind == Kind::Diagram)
                && n.family.is_some() == (n.kind == Kind::Diagram),
            "childRef/family mismatch"
        );
        if n.kind == Kind::Diagram {
            *count += 1;
            ensure!(*count <= 8, "Child budget");
            ensure!(
                n.child_ref.as_ref() == Some(&n.id),
                "Child reference identity mismatch"
            );
            if !["flowchart", "sequenceDiagram", "stateDiagram-v2"]
                .contains(&n.family.as_deref().unwrap_or(""))
            {
                bail!("Unsupported child family");
            }
        }
        for c in &n.children {
            visit(c, doc, ids, count, depth + 1)?;
        }
        Ok(())
    }
    ensure!(
        matches!(doc.root.kind, Kind::Row | Kind::Column),
        "Root must be a region"
    );
    visit(&doc.root, doc, &mut ids, &mut children, 1)
}
