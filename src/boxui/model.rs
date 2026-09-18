use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Row,
    Column,
    Text,
    Value,
    Button,
    Input,
    Diagram,
}
impl Kind {
    pub fn is_region(self) -> bool {
        matches!(self, Self::Row | Self::Column)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    String,
    Number,
    Boolean,
    None,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BindingRole {
    Value,
    Command,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub id: String,
    pub role: BindingRole,
    #[serde(rename = "type")]
    pub data_type: DataType,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Size {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grow: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Node {
    pub id: String,
    pub kind: Kind,
    pub version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_binding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_binding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
}
impl Node {
    pub fn children(&self) -> &[Node] {
        self.children.as_deref().unwrap_or(&[])
    }
    pub fn grow(&self) -> f64 {
        self.size.as_ref().and_then(|s| s.grow).unwrap_or(
            if self.kind.is_region() || self.kind == Kind::Diagram {
                1.0
            } else {
                0.0
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BoxUiDocument {
    pub profile: String,
    pub document_id: String,
    pub bindings: Vec<Binding>,
    pub root: Node,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChildSource {
    #[serde(rename = "ref")]
    pub reference: String,
    pub family: String,
    pub source: String,
    pub source_start: usize,
    pub source_end: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParsedBoxUi {
    pub contract: String,
    pub model: BoxUiDocument,
    pub child_sources: Vec<ChildSource>,
    pub diagnostics: Vec<super::Diagnostic>,
}

/// Fixed registry for this profile. Unknown widgets cannot silently fall back.
pub struct WidgetRegistry;
impl WidgetRegistry {
    pub const BUILTINS: [(Kind, u32); 7] = [
        (Kind::Row, 1),
        (Kind::Column, 1),
        (Kind::Text, 1),
        (Kind::Value, 1),
        (Kind::Button, 1),
        (Kind::Input, 1),
        (Kind::Diagram, 1),
    ];
    pub fn supports(kind: Kind, version: u32) -> bool {
        Self::BUILTINS.contains(&(kind, version))
    }
}
