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

fn minimal(children: Vec<serde_json::Value>) -> serde_json::Value {
    serde_json::json!({"profile":"boxui/0.1","documentId":"limits","bindings":[],
        "root":{"id":"root","kind":"column","version":1,"children":children}})
}
fn text_node(id: String) -> serde_json::Value {
    serde_json::json!({"id":id,"kind":"text","version":1,"text":"test"})
}
fn parse_value(v: &serde_json::Value) -> Result<ParsedBoxUi> {
    parse_boxui(&format!("boxui 0.1\n{v}"))
}

#[test]
fn source_node_depth_text_and_child_limits_have_exact_boundaries() {
    for count in [255, 256, 257] {
        let v = minimal((1..count).map(|i| text_node(format!("n{i}"))).collect());
        assert_eq!(parse_value(&v).is_ok(), count <= 256, "nodes {count}");
    }
    for depth in [15, 16, 17] {
        let mut node = text_node("leaf".into());
        for i in 1..depth {
            node = serde_json::json!({"id":format!("n{i}"),"kind":"column","version":1,"children":[node]});
        }
        let mut v = minimal(vec![]);
        v["root"] = node;
        assert_eq!(parse_value(&v).is_ok(), depth <= 16, "depth {depth}");
    }
    for chars in [4095, 4096, 4097] {
        let mut v = minimal(vec![text_node("text".into())]);
        v["root"]["children"][0]["text"] = serde_json::json!("ø".repeat(chars));
        assert_eq!(
            parse_value(&v).is_ok(),
            chars <= 4096,
            "Unicode scalars {chars}"
        );
    }
    for bytes in [MAX_SOURCE_BYTES - 1, MAX_SOURCE_BYTES, MAX_SOURCE_BYTES + 1] {
        let mut s = format!("boxui 0.1\n{}", minimal(vec![text_node("text".into())]));
        s.push_str(&" ".repeat(bytes - s.len()));
        assert_eq!(
            parse_boxui(&s).is_ok(),
            bytes <= MAX_SOURCE_BYTES,
            "source bytes {bytes}"
        );
    }
    for count in [7, 8, 9] {
        let v=minimal((0..count).map(|i|serde_json::json!({"id":format!("d{i}"),"kind":"diagram","version":1,"label":"Diagram","family":"flowchart","source":"flowchart LR\nA-->B"})).collect());
        assert_eq!(parse_value(&v).is_ok(), count <= 8, "diagram count {count}");
    }
    for bytes in [65535, 65536, 65537] {
        let v = minimal(vec![
            serde_json::json!({"id":"d","kind":"diagram","version":1,"label":"Diagram","family":"flowchart","source":"x".repeat(bytes)}),
        ]);
        assert_eq!(
            parse_value(&v).is_ok(),
            bytes <= 65536,
            "child bytes {bytes}"
        );
    }
}

#[test]
fn active_or_ambiguous_child_svg_is_rejected_locally() {
    for body in [
        "<image href=\"https://example.invalid/a.png\"/>",
        "<foreignObject><p>unsafe</p></foreignObject>",
        "<rect onclick=\"alert(1)\"/>",
        "<rect fill=\"url(https://example.invalid/a)\"/>",
        "<rect style=\"fill:u\\72l(https://example.invalid/a)\"/>",
        "<style>rect { fill:red }</style>",
        "<rect id=\"duplicate\"/><rect id=\"duplicate\"/>",
        "<rect clip-path=\"url(#missing)\"/>",
        "<use href=\"#recursive\" id=\"recursive\"/>",
    ] {
        let mut r = request();
        r.children[0] = PreparedChild::Svg {
            reference: "state-view".into(),
            width: 100.0,
            height: 100.0,
            svg: format!("<svg xmlns=\"http://www.w3.org/2000/svg\">{body}</svg>"),
        };
        let f = prepare_boxui(&r, &MonospaceMetrics).unwrap();
        assert_eq!(f.diagnostics[0].code, "child-svg-unsupported", "{body}");
        assert_eq!(f.controls.len(), 3);
    }
    let mut r = request();
    r.child_profiles.clear();
    assert_eq!(
        prepare_boxui(&r, &MonospaceMetrics).unwrap().diagnostics[0].code,
        "child-profile"
    );
}

#[test]
fn missing_stale_disabled_and_simulated_are_explicit() {
    let mut r = request();
    r.snapshot.values[0].validity = Validity::Missing;
    r.snapshot.values[0].value = serde_json::Value::Null;
    r.snapshot.values[1].validity = Validity::Stale;
    r.snapshot.commands[1].enabled = false;
    r.snapshot.commands[1].reason = "Paused".into();
    let f = prepare_boxui(&r, &MonospaceMetrics).unwrap();
    assert!(f.static_svg.contains("unbound / missing"));
    assert!(f.static_svg.contains("[stale]"));
    assert!(f.static_svg.contains("disabled: Paused"));
    assert!(!f.controls[1].enabled);
}

#[test]
fn growth_caps_redistribute_space_without_changing_source_order() {
    let mut r = request();
    r.children.clear();
    r.snapshot.values.clear();
    r.snapshot.commands.clear();
    let v = serde_json::json!({"profile":"boxui/0.1","documentId":"activity-demo","bindings":[],"root":{
    "id":"row","kind":"row","version":1,"children":[
        {"id":"first","kind":"text","version":1,"text":"A","size":{"min":80,"max":100,"grow":1}},
        {"id":"second","kind":"text","version":1,"text":"B","size":{"min":80,"grow":1}}
    ]}});
    r.model = serde_json::from_value(v).unwrap();
    let l = layout_boxui(&r, &MonospaceMetrics).unwrap();
    assert_eq!(l.items[1].id, "first");
    assert_eq!(l.items[1].rect.width, 100.0);
    assert_eq!(l.items[2].rect.x, 116.0);
    assert_eq!(l.items[2].rect.width, 516.0);
}

#[test]
fn all_initial_child_families_compose_real_renderer_output() {
    for (family, source) in [
        ("flowchart", "flowchart LR\nA-->B"),
        ("sequenceDiagram", "sequenceDiagram\nA->>B: Hello"),
        (
            "stateDiagram-v2",
            "stateDiagram-v2\n[*] --> Running\nRunning --> Suspended: Suspend",
        ),
    ] {
        let mut r = request();
        r.model.root.children.as_mut().unwrap()[4].family = Some(family.into());
        let svg = mermaid_rs_renderer::render(source).unwrap();
        r.children[0] = PreparedChild::Svg {
            reference: "state-view".into(),
            width: 400.0,
            height: 200.0,
            svg,
        };
        let f = prepare_boxui(&r, &MonospaceMetrics).unwrap();
        assert!(f.diagnostics.is_empty(), "{family}: {:?}", f.diagnostics);
    }
}

#[test]
fn preparation_is_isolated_between_parallel_sessions() {
    let threads: Vec<_> = (0..8)
        .map(|i| {
            std::thread::spawn(move || {
                let mut r = request();
                r.key.session = format!("session-{i}");
                let frame = prepare_boxui(&r, &MonospaceMetrics).unwrap();
                assert_eq!(frame.key, r.key);
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}

#[test]
fn deadline_and_frame_budget_are_enforced() {
    struct SlowMetrics;
    impl TextMetrics for SlowMetrics {
        fn measure(&self, text: &str) -> TextExtent {
            std::thread::sleep(std::time::Duration::from_millis(5));
            MonospaceMetrics.measure(text)
        }
        fn font_family(&self) -> &str {
            "monospace"
        }
        fn font_size(&self) -> f64 {
            14.0
        }
    }
    let mut r = request();
    r.budget_ms = 1;
    let err = prepare_boxui(&r, &SlowMetrics).unwrap_err();
    assert_eq!(err.diagnostic.code, "budget-exceeded");
    assert_eq!(err.status_code(), 3);
    r.budget_ms = 10000;
    r.children[0] = PreparedChild::Svg {
        reference: "state-view".into(),
        width: 100.0,
        height: 100.0,
        svg: format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\"><text>{}</text></svg>",
            "x".repeat(MAX_FRAME_BYTES / 2)
        ),
    };
    let err = prepare_boxui(&r, &MonospaceMetrics).unwrap_err();
    assert_eq!(err.diagnostic.code, "frame-budget");
    assert_eq!(err.status_code(), 3);
}

#[test]
fn wire_decoding_rejects_duplicate_children_and_incomplete_snapshots() {
    let original: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../SDP/09--Verification/fixtures/prepare.json"
    ))
    .unwrap();
    for mode in 0..5 {
        let mut v = original.clone();
        match mode {
            0 => {
                let c = v["children"][0].clone();
                v["children"].as_array_mut().unwrap().push(c);
            }
            1 => {
                v["snapshot"]["values"].as_array_mut().unwrap().pop();
            }
            2 => {
                v["snapshot"]["values"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("value");
            }
            3 => {
                v["children"][0]["error"] = serde_json::json!("conflicts with success");
            }
            _ => {
                v["extra"] = serde_json::json!(true);
            }
        }
        assert!(
            decode_prepare_json(&serde_json::to_vec(&v).unwrap()).is_err(),
            "mode {mode}"
        );
    }
}

#[test]
fn truncated_unicode_sources_never_panic_or_report_out_of_bounds_spans() {
    let s = format!(
        "boxui 0.1\n{}",
        minimal(vec![
            serde_json::json!({"id":"text","kind":"text","version":1,"text":"æøå 日本語 \\\""})
        ])
    );
    for end in 0..s.len() {
        if let Err(e) = parse_boxui_bytes(&s.as_bytes()[..end]) {
            if let (Some(a), Some(b)) = (e.diagnostic.source_start, e.diagnostic.source_end) {
                assert!(a <= b && b <= end, "range {a}..{b}, input length {end}");
            }
        }
    }
    let s = "boxui 0.1\n{\"x\":\"\\";
    let e = parse_boxui(s).unwrap_err();
    assert!(e.diagnostic.source_end.unwrap() <= s.len());
}

#[test]
fn fallback_font_and_large_host_metrics_grow_edit_and_footer_geometry() {
    struct Fallback;
    impl TextMetrics for Fallback {
        fn measure(&self, text: &str) -> TextExtent {
            TextExtent {
                width: text.chars().count() as f64 * 10.0,
                height: if text.contains('語') { 48.0 } else { 32.0 },
                baseline: if text.contains('語') { 38.0 } else { 24.0 },
            }
        }
        fn font_family(&self) -> &str {
            "monospace"
        }
        fn font_size(&self) -> f64 {
            24.0
        }
    }
    let mut r = request();
    r.viewport = Viewport {
        width: 1280.0,
        height: 960.0,
    };
    r.snapshot.values[1].value = serde_json::json!("日本語");
    let layout = layout_boxui(&r, &Fallback).unwrap();
    let edit = layout
        .items
        .iter()
        .find(|i| i.id == "context-input")
        .unwrap()
        .content_rect
        .unwrap();
    assert!(edit.height >= 64.0);
    let (footer, baseline) = layout.simulation_footer.unwrap();
    assert_eq!(footer.height, 40.0);
    assert!(baseline <= footer.height);
    let f = prepare_boxui(&r, &Fallback).unwrap();
    assert_eq!(f.controls[0].rect, edit);
}
