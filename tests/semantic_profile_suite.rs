#![cfg(feature = "libavoid")]
use mermaid_rs_renderer::{
    LayoutConfig, Theme, layout::compute_semantic_layout, parse_mermaid_strict,
    routing_backend::RoutingControl,
};
use std::time::{Duration, Instant};
#[test]
fn native_domain_types_survive_orthogonal_routing() {
    for source in [
        "classDiagram\nclass Unit\nclass Functionality\nUnit \"1\" *-- \"0..*\" Functionality : owns",
        "stateDiagram-v2\n[*] --> Missing\nMissing --> Current : observation\nCurrent --> Missing : reset",
        "erDiagram\nSOURCE ||--o{ OCCURRENCE : provenance",
        "requirementDiagram\nrequirement R {\nid: R1\ntext: Import\n}\nelement Test {\ntype: test\n}\nTest - verifies -> R",
    ] {
        let graph = parse_mermaid_strict(source).unwrap().graph;
        let mut config = LayoutConfig::default();
        config.node_spacing = 70.;
        config.rank_spacing = 80.;
        let (layout, _) = compute_semantic_layout(
            &graph,
            &Theme::modern(),
            &config,
            &RoutingControl {
                deadline: Instant::now() + Duration::from_secs(10),
                cancelled: &|| false,
            },
        )
        .unwrap();
        assert_eq!(layout.kind, graph.kind);
        assert_eq!(layout.edges.len(), graph.edges.len());
        for e in &layout.edges {
            for p in e.points.windows(2) {
                assert!((p[0].0 - p[1].0).abs() < 0.01 || (p[0].1 - p[1].1).abs() < 0.01);
            }
        }
        let svg = mermaid_rs_renderer::render_svg(&layout, &Theme::modern(), &config);
        assert!(svg.contains("<text"));
    }
}
#[test]
fn crowfeet_extend_outside_both_entity_boundaries() {
    let graph = parse_mermaid_strict("erDiagram\nA ||--o{ B : owns")
        .unwrap()
        .graph;
    let layout =
        mermaid_rs_renderer::compute_layout(&graph, &Theme::modern(), &LayoutConfig::default());
    let svg = mermaid_rs_renderer::render_svg(&layout, &Theme::modern(), &LayoutConfig::default());
    assert!(svg.contains("M 0 -6 L 10 0 L 0 6"));
    assert!(svg.contains("cx=\"18\""));
}

#[test]
fn cyclic_currentness_and_locked_self_loop_ports() {
    let source = include_str!("fixtures/semantic/currentness.mmd");
    let graph = parse_mermaid_strict(source).unwrap().graph;
    let mut config = LayoutConfig::default();
    config.max_label_width_chars = 20;
    let (layout, diagnostics) = compute_semantic_layout(
        &graph,
        &Theme::modern(),
        &config,
        &RoutingControl {
            deadline: Instant::now() + Duration::from_secs(10),
            cancelled: &|| false,
        },
    )
    .unwrap();
    assert_eq!(layout.edges.len(), 8);
    for e in &layout.edges {
        for p in e.points.windows(2) {
            assert!((p[0].0 - p[1].0).abs() < 0.01 || (p[0].1 - p[1].1).abs() < 0.01);
        }
    }
    // Native pin competition can choose another equally valid route. Verify
    // semantic and geometric invariants, not byte-identical SVG (known gap).
    for _ in 0..24 {
        let (again, _) = compute_semantic_layout(
            &graph,
            &Theme::modern(),
            &config,
            &RoutingControl {
                deadline: Instant::now() + Duration::from_secs(10),
                cancelled: &|| false,
            },
        )
        .unwrap();
        assert_eq!(again.edges.len(), layout.edges.len());
        for (a, b) in again.edges.iter().zip(&layout.edges) {
            assert_eq!(
                (&a.from, &a.to, a.label.as_ref().map(|l| &l.lines)),
                (&b.from, &b.to, b.label.as_ref().map(|l| &l.lines))
            );
            for p in a.points.windows(2) {
                assert!((p[0].0 - p[1].0).abs() < 0.01 || (p[0].1 - p[1].1).abs() < 0.01);
            }
        }
    }
    assert!(
        layout.width < 1800.,
        "unbounded empty rank bands: {} {diagnostics:?}",
        layout.width
    );
}
