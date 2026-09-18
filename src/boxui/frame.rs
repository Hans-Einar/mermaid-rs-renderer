use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FrameKey {
    pub session: String,
    pub epoch: String,
    pub block_id: String,
    pub incarnation: String,
    pub source_revision: String,
    pub binding_revision: String,
    pub state_revision: String,
    pub viewport_revision: String,
    pub theme_revision: String,
    pub frame_sequence: String,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Palette {
    pub background: String,
    pub foreground: String,
    pub surface: String,
    pub border: String,
    pub accent: String,
    pub muted: String,
    pub error: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Validity {
    Missing,
    Current,
    Stale,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provenance {
    Simulated,
    Native,
    Unbound,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValueState {
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: DataType,
    pub value: Value,
    pub validity: Validity,
    pub revision: String,
    pub source_session: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CommandState {
    pub id: String,
    pub argument_type: DataType,
    pub enabled: bool,
    pub reason: String,
    pub provenance: Provenance,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Snapshot {
    pub context_revision: String,
    pub values: Vec<ValueState>,
    pub commands: Vec<CommandState>,
}
impl Snapshot {
    pub(crate) fn value(&self, id: &str) -> &ValueState {
        self.values
            .iter()
            .find(|v| v.id == id)
            .expect("validated binding")
    }
    pub(crate) fn command(&self, id: &str) -> &CommandState {
        self.commands
            .iter()
            .find(|v| v.id == id)
            .expect("validated binding")
    }
    pub(crate) fn simulated(&self) -> bool {
        self.commands
            .iter()
            .any(|c| c.provenance == Provenance::Simulated)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum PreparedChild {
    Svg {
        #[serde(rename = "ref")]
        reference: String,
        width: f64,
        height: f64,
        svg: String,
    },
    Error {
        #[serde(rename = "ref")]
        reference: String,
        error: String,
    },
}
impl PreparedChild {
    pub fn reference(&self) -> &str {
        match self {
            Self::Svg { reference, .. } | Self::Error { reference, .. } => reference,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PrepareRequest {
    pub contract: String,
    pub key: FrameKey,
    pub model: BoxUiDocument,
    pub snapshot: Snapshot,
    pub viewport: Viewport,
    pub palette: Palette,
    pub font_signature: String,
    pub budget_ms: u64,
    pub child_profiles: Vec<String>,
    pub children: Vec<PreparedChild>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Control {
    pub id: String,
    pub kind: Kind,
    pub version: u32,
    pub rect: Rect,
    pub clip: Rect,
    pub enabled: bool,
    pub role: String,
    pub accessible_name: String,
    pub command_binding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_binding: Option<String>,
    pub value_type: DataType,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BoxUiFrame {
    pub contract: String,
    pub key: FrameKey,
    pub width: f64,
    pub height: f64,
    pub static_svg: String,
    pub preview_svg: String,
    pub controls: Vec<Control>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Use at an untrusted JSON boundary: serde alone does not reject duplicate keys.
pub fn decode_prepare_json(bytes: &[u8]) -> Result<PrepareRequest> {
    if bytes.len() > MAX_FRAME_BYTES {
        return Err(BoxUiError::new(
            "request-budget",
            "Prepare envelope exceeds 8 MiB",
        ));
    }
    let s =
        std::str::from_utf8(bytes).map_err(|e| BoxUiError::new("request-utf8", e.to_string()))?;
    let (value, _) = super::parse::strict_json(s, 0)?;
    if let Some(model) = value.get("model") {
        super::validate::raw_model(model)?;
    }
    // serde_json::Value accepts missing as null; the wire contract requires an explicit value.
    if value
        .pointer("/snapshot/values")
        .and_then(Value::as_array)
        .is_some_and(|a| a.iter().any(|v| v.get("value").is_none()))
    {
        return Err(BoxUiError::new(
            "snapshot-shape",
            "Every value needs an explicit value field",
        ));
    }
    let r: PrepareRequest = serde_json::from_value(value)
        .map_err(|e| BoxUiError::new("request-shape", e.to_string()))?;
    validate_request(&r)?;
    Ok(r)
}

pub(crate) struct Budget<'a> {
    started: Instant,
    duration: Duration,
    cancelled: &'a dyn Fn() -> bool,
}
impl<'a> Budget<'a> {
    pub(crate) fn new(ms: u64, cancelled: &'a dyn Fn() -> bool) -> Self {
        Self {
            started: Instant::now(),
            duration: Duration::from_millis(ms),
            cancelled,
        }
    }
    pub(crate) fn check(&self) -> Result<()> {
        if (self.cancelled)() {
            return Err(BoxUiError::new("cancelled", "Preparation cancelled"));
        }
        if self.started.elapsed() > self.duration {
            return Err(BoxUiError::new(
                "budget-exceeded",
                "Preparation time budget exceeded",
            ));
        }
        Ok(())
    }
}
pub(crate) fn validate_request(r: &PrepareRequest) -> Result<()> {
    validate_boxui(&r.model)?;
    let fail = |s| BoxUiError::new("invalid-request", s);
    if r.contract != CONTRACT {
        return Err(BoxUiError::new(
            "unsupported-contract",
            "Expected BX-HOST/0.1-draft1",
        ));
    }
    if !(1..=10000).contains(&r.budget_ms) || r.font_signature.is_empty() {
        return Err(fail("Invalid budget or empty font signature"));
    }
    if r.key.session.is_empty()
        || r.key.session.chars().count() > 128
        || r.key.block_id != r.model.document_id
    {
        return Err(fail("Invalid session or block identity"));
    }
    for s in [
        &r.key.epoch,
        &r.key.incarnation,
        &r.key.source_revision,
        &r.key.binding_revision,
        &r.key.state_revision,
        &r.key.viewport_revision,
        &r.key.theme_revision,
        &r.key.frame_sequence,
    ] {
        if s.parse::<u64>().ok().is_none_or(|v| v.to_string() != *s) {
            return Err(fail("Frame counters must be canonical decimal u64 strings"));
        }
    }
    if !r.viewport.width.is_finite()
        || !r.viewport.height.is_finite()
        || !(320.0..=8192.0).contains(&r.viewport.width)
        || !(240.0..=8192.0).contains(&r.viewport.height)
    {
        return Err(BoxUiError::new(
            "layout-no-space",
            "Usable viewport must be 320..8192 by 240..8192 px",
        ));
    }
    for c in [
        &r.palette.background,
        &r.palette.foreground,
        &r.palette.surface,
        &r.palette.border,
        &r.palette.accent,
        &r.palette.muted,
        &r.palette.error,
    ] {
        if c.len() != 9
            || !c.starts_with('#')
            || !c.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
        {
            return Err(fail("Palette entries must be #RRGGBBAA"));
        }
    }
    let bindings: BTreeMap<_, _> = r
        .model
        .bindings
        .iter()
        .map(|b| (b.id.as_str(), b))
        .collect();
    let mut seen = BTreeSet::new();
    for v in &r.snapshot.values {
        let b = bindings
            .get(v.id.as_str())
            .ok_or_else(|| fail("Unknown snapshot value"))?;
        if !seen.insert(v.id.as_str()) || b.role != BindingRole::Value || b.data_type != v.data_type
        {
            return Err(fail("Duplicate or incompatible snapshot value"));
        }
        let valid = if v.validity == Validity::Missing {
            v.value.is_null()
        } else {
            match v.data_type {
                DataType::String => v.value.as_str().is_some_and(|s| s.chars().count() <= 4096),
                DataType::Number => v.value.as_f64().is_some_and(f64::is_finite),
                DataType::Boolean => v.value.is_boolean(),
                DataType::None => false,
            }
        };
        if !valid {
            return Err(fail(
                "Snapshot value has wrong type, validity or text length",
            ));
        }
    }
    for c in &r.snapshot.commands {
        let b = bindings
            .get(c.id.as_str())
            .ok_or_else(|| fail("Unknown snapshot command"))?;
        if !seen.insert(c.id.as_str())
            || b.role != BindingRole::Command
            || b.data_type != c.argument_type
            || !matches!(c.argument_type, DataType::String | DataType::None)
        {
            return Err(fail("Duplicate or incompatible snapshot command"));
        }
        if c.provenance == Provenance::Unbound && c.enabled {
            return Err(fail("Unbound command cannot be enabled"));
        }
        if c.reason.chars().count() > 4096 {
            return Err(fail("Command reason exceeds text limit"));
        }
    }
    if seen.len() != bindings.len() {
        return Err(fail("Snapshot must resolve every binding exactly once"));
    }
    let profiles: BTreeSet<_> = r.child_profiles.iter().collect();
    if profiles.len() != r.child_profiles.len() {
        return Err(fail("Duplicate child profile"));
    }
    fn refs<'a>(n: &'a Node, out: &mut BTreeSet<&'a str>) {
        if let Some(id) = &n.child_ref {
            out.insert(id);
        }
        for c in n.children() {
            refs(c, out);
        }
    }
    let mut expected = BTreeSet::new();
    refs(&r.model.root, &mut expected);
    let mut actual = BTreeSet::new();
    for c in &r.children {
        if !actual.insert(c.reference()) || !expected.contains(c.reference()) {
            return Err(fail("Duplicate or unexpected prepared child"));
        }
        if let PreparedChild::Svg {
            width, height, svg, ..
        } = c
        {
            if !width.is_finite()
                || !height.is_finite()
                || !(1.0..=8192.0).contains(width)
                || !(1.0..=8192.0).contains(height)
                || svg.len() > MAX_FRAME_BYTES
            {
                return Err(fail("Invalid prepared child size"));
            }
        }
        if let PreparedChild::Error { error, .. } = c {
            if error.chars().count() > 4096 {
                return Err(fail("Child diagnostic exceeds text limit"));
            }
        }
    }
    if expected != actual {
        return Err(fail(
            "Each diagram requires exactly one prepared child or error",
        ));
    }
    Ok(())
}
