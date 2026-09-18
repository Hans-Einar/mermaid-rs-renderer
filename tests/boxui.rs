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

fn request() -> PrepareRequest {
    decode_prepare_json(include_bytes!(
        "../SDP/09--Verification/fixtures/prepare.json"
    ))
    .unwrap()
}
#[test]
fn prepares_coherent_deterministic_frame_from_handoff() {
    let r = request();
    let frame = prepare_boxui(&r, &MonospaceMetrics).unwrap();
    assert_eq!(frame, prepare_boxui(&r, &MonospaceMetrics).unwrap());
    assert_eq!(frame.key, r.key);
    assert_eq!(
        frame
            .controls
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        ["context-input", "pause", "continue"]
    );
    assert!(frame.static_svg.contains(">C1</text>"));
    assert!(!frame.preview_svg.contains(">C1</text>"));
    assert!(frame.static_svg.contains("simulated snapshot"));
    assert!(frame.static_svg.contains("Mock child scene"));
    assert!(frame.diagnostics.is_empty(), "{:?}", frame.diagnostics);
    for c in frame.controls {
        assert_eq!(c.rect, c.clip);
        assert!(c.rect.x >= 0.0 && c.rect.y >= 0.0);
        assert!(c.rect.x + c.rect.width <= frame.width);
        assert!(c.rect.y + c.rect.height <= frame.height);
    }
    roxmltree::Document::parse(&frame.static_svg).unwrap();
    roxmltree::Document::parse(&frame.preview_svg).unwrap();
}
#[test]
fn rejects_snapshot_and_identity_ambiguity() {
    let original: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../SDP/09--Verification/fixtures/prepare.json"
    ))
    .unwrap();
    for (path, value) in [
        ("/key/epoch", serde_json::json!("01")),
        ("/key/epoch", serde_json::json!("18446744073709551616")),
        ("/key/blockId", serde_json::json!("other")),
        ("/snapshot/values/0/value", serde_json::json!("42")),
        ("/snapshot/values/0/validity", serde_json::json!("missing")),
        (
            "/snapshot/commands/0/provenance",
            serde_json::json!("unbound"),
        ),
        ("/model/root/children/4/size", serde_json::json!(null)),
        ("/children/0/width", serde_json::json!(0)),
        ("/palette/background", serde_json::json!("red")),
    ] {
        let mut v = original.clone();
        *v.pointer_mut(path)
            .unwrap_or_else(|| panic!("missing {path}")) = value;
        assert!(
            decode_prepare_json(&serde_json::to_vec(&v).unwrap()).is_err(),
            "accepted {path}"
        );
    }
}
#[test]
fn malformed_child_stays_local_and_text_is_literal() {
    let mut r = request();
    r.model.root.children.as_mut().unwrap()[0].text = Some("<script>alert & stuff</script>".into());
    r.children[0] = PreparedChild::Svg {
        reference: "state-view".into(),
        width: 100.0,
        height: 100.0,
        svg: "<svg xmlns=\"http://www.w3.org/2000/svg\"><script>alert(1)</script></svg>".into(),
    };
    let frame = prepare_boxui(&r, &MonospaceMetrics).unwrap();
    assert!(frame.static_svg.contains("&lt;script&gt;"));
    assert!(!frame.static_svg.contains("<script>"));
    assert!(frame.static_svg.contains("Diagram unavailable"));
    assert_eq!(frame.controls.len(), 3);
    assert_eq!(frame.diagnostics[0].node_id.as_deref(), Some("state-view"));
}
#[test]
fn child_ids_and_local_references_are_namespaced() {
    let mut r = request();
    r.children[0]=PreparedChild::Svg{reference:"state-view".into(),width:100.0,height:100.0,
        svg:r##"<svg xmlns="http://www.w3.org/2000/svg"><defs><clipPath id="original"><rect width="10" height="10"/></clipPath></defs><g clip-path="url(#original)"><text>Safe</text></g></svg>"##.into()};
    let f = prepare_boxui(&r, &MonospaceMetrics).unwrap();
    assert!(f.diagnostics.is_empty(), "{:?}", f.diagnostics);
    assert!(f.static_svg.contains("url(#bx-child-0-0)"));
    assert!(!f.static_svg.contains("#original"));
}
#[test]
fn cancellation_metrics_and_no_space_fail_without_partial_frames() {
    let r = request();
    assert_eq!(
        prepare_boxui_cancellable(&r, &MonospaceMetrics, &|| true)
            .unwrap_err()
            .diagnostic
            .code,
        "cancelled"
    );
    let mut small = r.clone();
    small.viewport.height = 240.0;
    assert_eq!(
        prepare_boxui(&small, &MonospaceMetrics)
            .unwrap_err()
            .diagnostic
            .code,
        "layout-no-space"
    );
    struct BadMetrics;
    impl TextMetrics for BadMetrics {
        fn measure(&self, _: &str) -> TextExtent {
            TextExtent {
                width: f64::NAN,
                height: 10.0,
                baseline: 8.0,
            }
        }
        fn font_family(&self) -> &str {
            "test"
        }
        fn font_size(&self) -> f64 {
            14.0
        }
    }
    assert_eq!(
        prepare_boxui(&r, &BadMetrics).unwrap_err().diagnostic.code,
        "invalid-metrics"
    );
}
