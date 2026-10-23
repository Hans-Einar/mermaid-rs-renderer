//! BoxUI 0.1: a bounded UI description, independent of Mermaid's graph AST.
//! Host integration is described in `SDP/06--Container-Design/06-02--XFMD-Host-Contract.md`.
mod embedded;
mod frame;
mod layout;
mod model;
mod parse;
mod svg;
mod validate;
pub use frame::*;
pub use layout::{
    BoxUiLayout, LayoutItem, MonospaceMetrics, TextExtent, TextMetrics, layout_boxui,
};
pub use model::*;
pub use parse::{parse_boxui, parse_boxui_bytes};
pub use svg::{prepare_boxui, prepare_boxui_cancellable};
pub use validate::validate_boxui;

pub const CONTRACT: &str = "BX-HOST/0.1-draft1";
pub const MAX_SOURCE_BYTES: usize = 256 * 1024;
pub const MAX_FRAME_BYTES: usize = 8 * 1024 * 1024;
/// Host capability identifiers frozen by BX-HOST/0.1-draft1.
pub const CHILD_PROFILES: [(&str, &str); 3] = [
    ("flowchart", "XFMD Flowchart 1"),
    ("sequenceDiagram", "XFMD Sequence 2"),
    ("stateDiagram-v2", "XFMD State 1"),
];

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Diagnostic {
    pub code: String,
    pub severity: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_end: Option<usize>,
}

#[derive(Debug, Clone, serde::Serialize, thiserror::Error)]
#[error("{diagnostic:?}")]
pub struct BoxUiError {
    pub diagnostic: Diagnostic,
}
pub type Result<T> = std::result::Result<T, BoxUiError>;

impl BoxUiError {
    /// Draft1 host status category; local child diagnostics still produce a successful frame.
    pub fn status_code(&self) -> u32 {
        match self.diagnostic.code.as_str() {
            "cancelled" => 4,
            "unsupported-profile" | "unsupported-contract" | "widget-version" | "child-family" => 2,
            "frame-encoding" => 5,
            code if code.contains("budget") => 3,
            _ => 1,
        }
    }
    pub(crate) fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            diagnostic: Diagnostic {
                code: code.into(),
                severity: "error".into(),
                message: message.into().chars().take(4096).collect(),
                node_id: None,
                source_start: None,
                source_end: None,
            },
        }
    }
    pub(crate) fn node(mut self, id: &str) -> Self {
        if validate::identifier(id) {
            self.diagnostic.node_id = Some(id.into());
        }
        self
    }
    pub(crate) fn span(mut self, start: usize, end: usize) -> Self {
        self.diagnostic.source_start = Some(start);
        self.diagnostic.source_end = Some(end);
        self
    }
}
