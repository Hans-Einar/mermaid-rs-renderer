use super::*;
use std::collections::BTreeMap;
fn source() -> String {
    format!(
        "boxui 0.1\n{}",
        include_str!("../../SDP/09--Verification/fixtures/activity.boxui.json")
    )
}
fn metrics(s: &str) -> (f64, f64) {
    (s.chars().count() as f64 * 8., 20.)
}
#[test]
fn source_preserves_typed_bindings_and_extracts_children() {
    let p = parse(&source()).unwrap();
    assert_eq!(p.child_sources.len(), 1);
    assert_eq!(p.model.root.children[2].kind, Kind::Input);
    assert_eq!(
        p.model.root.children[5 - 1].child_ref.as_deref(),
        Some("state-view")
    );
}
#[test]
fn rejects_duplicates_unknowns_and_mistyped_bindings() {
    for s in [
        source().replace("\"profile\":", "\"profile\":\"boxui/0.1\",\"profile\":"),
        source().replace("\"input\"", "\"html\""),
        source().replace(
            "\"commandBinding\": \"set-context\"",
            "\"commandBinding\": \"measurement\"",
        ),
    ] {
        assert!(parse(&s).is_err());
    }
}
#[test]
fn deterministic_frames_have_separate_edit_surface() {
    let p = parse(&source()).unwrap();
    let a = prepare(
        &p.model,
        &Snapshot::default(),
        &BTreeMap::new(),
        640.,
        480.,
        2000,
        &metrics,
        &|| false,
    )
    .unwrap();
    let b = prepare(
        &p.model,
        &Snapshot::default(),
        &BTreeMap::new(),
        640.,
        480.,
        2000,
        &metrics,
        &|| false,
    )
    .unwrap();
    assert_eq!(a.static_svg, b.static_svg);
    assert_ne!(a.static_svg, a.preview_svg);
    assert_eq!(a.controls.len(), 3);
    assert!(a.controls.iter().all(|c| !c.enabled));
    assert_eq!(a.diagnostics.len(), 1);
}
#[test]
fn layout_reports_no_space_and_cancellation() {
    let p = parse(&source()).unwrap();
    assert!(
        prepare(
            &p.model,
            &Snapshot::default(),
            &BTreeMap::new(),
            120.,
            80.,
            2000,
            &metrics,
            &|| false
        )
        .is_err()
    );
    assert!(
        prepare(
            &p.model,
            &Snapshot::default(),
            &BTreeMap::new(),
            640.,
            480.,
            2000,
            &metrics,
            &|| true
        )
        .is_err()
    );
}
#[test]
fn values_are_typed_and_escape_svg() {
    let p = parse(&source()).unwrap();
    let mut s = Snapshot::default();
    s.values.insert(
        "context".into(),
        ValueState {
            value: "<script>&\"".into(),
            validity: "current".into(),
            revision: "1".into(),
        },
    );
    s.enabled.insert("set-context".into(), true);
    let f = prepare(
        &p.model,
        &s,
        &BTreeMap::new(),
        640.,
        480.,
        2000,
        &metrics,
        &|| false,
    )
    .unwrap();
    assert!(f.static_svg.contains("&lt;script&gt;"));
    assert!(
        f.controls
            .iter()
            .find(|c| c.kind == Kind::Input)
            .unwrap()
            .enabled
    );
    s.values.get_mut("context").unwrap().value = 42.into();
    assert!(
        prepare(
            &p.model,
            &s,
            &BTreeMap::new(),
            640.,
            480.,
            2000,
            &metrics,
            &|| false
        )
        .is_err()
    );
}
#[test]
fn rejects_nonfinite_viewport() {
    let p = parse(&source()).unwrap();
    assert!(
        prepare(
            &p.model,
            &Snapshot::default(),
            &BTreeMap::new(),
            f64::NAN,
            480.,
            2000,
            &metrics,
            &|| false
        )
        .is_err()
    );
}
