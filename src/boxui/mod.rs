//! BoxUI 0.1: a bounded UI description, independent of Mermaid's graph AST.
//! Host integration is described in `SDP/06--Container-Design/06-02--XFMD-Host-Contract.md`.
mod model;
mod parse;
mod validate;
pub use model::*;
pub use parse::{parse_boxui, parse_boxui_bytes};
pub use validate::validate_boxui;

pub const CONTRACT: &str = "BX-HOST/0.1-draft1";
pub const MAX_SOURCE_BYTES: usize = 256 * 1024;
pub const MAX_FRAME_BYTES: usize = 8 * 1024 * 1024;

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
