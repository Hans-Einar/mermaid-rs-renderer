use super::model::Kind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub value_type: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValueState {
    pub value: serde_json::Value,
    pub validity: String,
    pub revision: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Snapshot {
    #[serde(default)]
    pub values: BTreeMap<String, ValueState>,
    #[serde(default)]
    pub enabled: BTreeMap<String, bool>,
    #[serde(default)]
    pub simulated: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChildScene {
    pub width: f64,
    pub height: f64,
    pub svg: String,
    #[serde(default)]
    pub error: String,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub width: f64,
    pub height: f64,
    pub static_svg: String,
    pub preview_svg: String,
    pub controls: Vec<Control>,
    pub diagnostics: Vec<String>,
}
