use super::*;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

pub fn parse_boxui_bytes(source: &[u8]) -> Result<ParsedBoxUi> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(BoxUiError::new("source-budget", "Source exceeds 256 KiB"));
    }
    parse_boxui(
        std::str::from_utf8(source).map_err(|e| BoxUiError::new("source-utf8", e.to_string()))?,
    )
}
pub fn parse_boxui(source: &str) -> Result<ParsedBoxUi> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(BoxUiError::new("source-budget", "Source exceeds 256 KiB"));
    }
    let mut offset = 0;
    let mut found = false;
    for line in source.split_inclusive('\n') {
        offset += line.len();
        if line.trim().is_empty() {
            continue;
        }
        if line.trim_end_matches(['\n', '\r']) != "boxui 0.1" {
            return Err(
                BoxUiError::new("unsupported-profile", "Expected exact header: boxui 0.1")
                    .span(offset - line.len(), offset),
            );
        }
        found = true;
        break;
    }
    if !found {
        return Err(BoxUiError::new("source-header", "Missing boxui 0.1 header"));
    }
    let (mut value, spans) = strict_json(&source[offset..], offset)?;
    let mut children = Vec::new();
    if let Some(root) = value.get_mut("root") {
        extract(root, "/root", &spans, &mut children).map_err(|e| e.span(offset, source.len()))?;
    }
    validate::raw_model(&value).map_err(|e| e.span(offset, source.len()))?;
    let model: BoxUiDocument = serde_json::from_value(value)
        .map_err(|e| BoxUiError::new("source-shape", e.to_string()).span(offset, source.len()))?;
    validate_boxui(&model).map_err(|mut e| {
        if let Some(id) = &e.diagnostic.node_id {
            if let Some(path) = find_node_path(&model.root, id, "/root") {
                if let Some(&(a, b)) = spans.get(&path) {
                    e = e.span(a, b);
                }
            }
        }
        if e.diagnostic.source_start.is_none() {
            e = e.span(offset, source.len());
        }
        e
    })?;
    Ok(ParsedBoxUi {
        contract: CONTRACT.into(),
        model,
        child_sources: children,
        diagnostics: vec![],
    })
}
fn find_node_path(node: &Node, id: &str, path: &str) -> Option<String> {
    if node.id == id {
        return Some(path.into());
    }
    node.children()
        .iter()
        .enumerate()
        .find_map(|(i, n)| find_node_path(n, id, &format!("{path}/children/{i}")))
}
fn extract(v: &mut Value, path: &str, spans: &Spans, out: &mut Vec<ChildSource>) -> Result<()> {
    if v.get("kind").and_then(Value::as_str) == Some("diagram") {
        let object = v.as_object_mut().unwrap();
        if object.contains_key("childRef") {
            return Err(BoxUiError::new(
                "source-shape",
                "childRef is generated; author source instead",
            ));
        }
        let id = object
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned();
        let family = object
            .get("family")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned();
        let source = object
            .remove("source")
            .and_then(|s| s.as_str().map(str::to_owned))
            .ok_or_else(|| {
                BoxUiError::new("source-shape", "Diagram requires a string source").node(&id)
            })?;
        if source.len() > 65536 {
            return Err(BoxUiError::new("child-budget", "Child source exceeds 64 KiB").node(&id));
        }
        let &(start, end) = spans.get(&format!("{path}/source")).unwrap();
        out.push(ChildSource {
            reference: id.clone(),
            family,
            source,
            source_start: start,
            source_end: end,
        });
        object.insert("childRef".into(), Value::String(id));
    }
    if let Some(children) = v.get_mut("children").and_then(Value::as_array_mut) {
        for (i, child) in children.iter_mut().enumerate() {
            extract(child, &format!("{path}/children/{i}"), spans, out)?;
        }
    }
    Ok(())
}

type Spans = BTreeMap<String, (usize, usize)>;
/// Strict JSON with duplicate-key rejection and original UTF-8 byte spans.
pub(crate) fn strict_json(s: &str, base: usize) -> Result<(Value, Spans)> {
    let mut p = Json {
        s,
        pos: 0,
        base,
        spans: BTreeMap::new(),
    };
    let value = p.value("", 0)?;
    p.ws();
    if p.pos != s.len() {
        return Err(p.error("Trailing bytes"));
    }
    Ok((value, p.spans))
}
struct Json<'a> {
    s: &'a str,
    pos: usize,
    base: usize,
    spans: Spans,
}
impl Json<'_> {
    fn ws(&mut self) {
        while self
            .s
            .as_bytes()
            .get(self.pos)
            .is_some_and(|c| matches!(c, b' ' | b'\n' | b'\r' | b'\t'))
        {
            self.pos += 1;
        }
    }
    fn error(&self, s: &str) -> BoxUiError {
        BoxUiError::new("invalid-json", s).span(self.base + self.pos, self.base + self.pos)
    }
    fn take(&mut self, c: u8) -> bool {
        self.ws();
        if self.s.as_bytes().get(self.pos) == Some(&c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn string(&mut self) -> Result<String> {
        self.ws();
        let start = self.pos;
        if !self.take(b'"') {
            return Err(self.error("Expected string"));
        }
        while self.pos < self.s.len() {
            let c = self.s.as_bytes()[self.pos];
            self.pos += 1;
            if c == b'\\' {
                self.pos = (self.pos + 1).min(self.s.len());
            } else if c == b'"' {
                return serde_json::from_str(&self.s[start..self.pos])
                    .map_err(|e| self.error(&e.to_string()));
            }
        }
        Err(self.error("Unterminated string"))
    }
    fn value(&mut self, path: &str, depth: usize) -> Result<Value> {
        if depth > 64 {
            return Err(self.error("JSON nesting exceeds 64"));
        }
        self.ws();
        let start = self.pos;
        let v = match self.s.as_bytes().get(self.pos).copied() {
            Some(b'{') => {
                self.pos += 1;
                let mut map = Map::new();
                if !self.take(b'}') {
                    loop {
                        let key = self.string()?;
                        if map.contains_key(&key) {
                            return Err(self.error("Duplicate object key"));
                        }
                        if !self.take(b':') {
                            return Err(self.error("Expected colon"));
                        }
                        let child = format!("{path}/{}", key.replace('~', "~0").replace('/', "~1"));
                        map.insert(key, self.value(&child, depth + 1)?);
                        if self.take(b'}') {
                            break;
                        }
                        if !self.take(b',') {
                            return Err(self.error("Expected comma"));
                        }
                    }
                }
                Value::Object(map)
            }
            Some(b'[') => {
                self.pos += 1;
                let mut a = vec![];
                if !self.take(b']') {
                    loop {
                        a.push(self.value(&format!("{path}/{}", a.len()), depth + 1)?);
                        if self.take(b']') {
                            break;
                        }
                        if !self.take(b',') {
                            return Err(self.error("Expected comma"));
                        }
                    }
                }
                Value::Array(a)
            }
            Some(b'"') => Value::String(self.string()?),
            Some(_) => {
                while self.s.as_bytes().get(self.pos).is_some_and(|c| {
                    !matches!(c, b',' | b']' | b'}' | b' ' | b'\n' | b'\r' | b'\t')
                }) {
                    self.pos += 1;
                }
                serde_json::from_str(&self.s[start..self.pos])
                    .map_err(|e| self.error(&e.to_string()))?
            }
            None => return Err(self.error("Expected JSON value")),
        };
        self.spans
            .insert(path.into(), (self.base + start, self.base + self.pos));
        Ok(v)
    }
}
