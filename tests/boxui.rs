use mermaid_rs_renderer::boxui::*;
const SOURCE: &str = include_str!("../SDP/09--Verification/fixtures/activity.boxui.json");
fn source() -> String {
    format!("boxui 0.1\n{SOURCE}")
}
#[test]
fn parses_handoff_and_preserves_child_source_byte_ranges() {
    let s = source();
    let parsed = parse_boxui(&s).unwrap();
    assert_eq!(parsed.contract, CONTRACT);
    assert_eq!(parsed.model.document_id, "activity-demo");
    let child = &parsed.child_sources[0];
    assert_eq!(
        serde_json::from_str::<String>(&s[child.source_start..child.source_end]).unwrap(),
        child.source
    );
    let expected: BoxUiDocument =
        serde_json::from_str(include_str!("../SDP/09--Verification/fixtures/model.json")).unwrap();
    assert_eq!(parsed.model, expected);
}
#[test]
fn rejects_ambiguous_or_invalid_sources() {
    for s in [
        source().replace("\"profile\":", "\"profile\":\"boxui/0.1\",\"profile\":"),
        source().replace("\"version\": 1", "\"version\": 2"),
        source().replace("\"kind\": \"text\"", "\"kind\": \"text\", \"label\": null"),
        format!("{} garbage", source()),
        source().replace("boxui 0.1", "boxui 0.2"),
        source().replace(
            "\"commandBinding\": \"suspend\"",
            "\"commandBinding\": \"measurement\"",
        ),
    ] {
        assert!(parse_boxui(&s).is_err(), "unexpected success: {s}");
    }
    assert!(parse_boxui_bytes(&[255]).is_err());
}
#[test]
fn rejects_unknown_fields_null_sizes_and_duplicate_ids() {
    let mut v: serde_json::Value = serde_json::from_str(SOURCE).unwrap();
    v["root"]["size"] = serde_json::json!({"grow":null});
    assert!(parse_boxui(&format!("boxui 0.1\n{v}")).is_err());
    v["root"].as_object_mut().unwrap().remove("size");
    v["root"]["children"][1]["id"] = v["root"]["children"][0]["id"].clone();
    let e = parse_boxui(&format!("boxui 0.1\n{v}")).unwrap_err();
    assert_eq!(e.diagnostic.code, "node-id");
    assert!(e.diagnostic.source_start.is_some());
}
