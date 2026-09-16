use mermaid_rs_renderer::layout::{flowchart_quality_metrics, validate_layout_invariants};
use mermaid_rs_renderer::{LayoutConfig, Theme, compute_layout, parse_mermaid_strict};

#[test]
fn traceability_routes_stay_short_without_endpoint_or_node_intrusions() {
    let graph = parse_mermaid_strict(include_str!(
        "fixtures/flowchart/routing-review/traceability.mmd"
    ))
    .unwrap()
    .graph;
    let layout = compute_layout(&graph, &Theme::modern(), &LayoutConfig::default());
    validate_layout_invariants(&layout).unwrap();
    let metrics = flowchart_quality_metrics(&layout).unwrap();
    assert_eq!(metrics.edge_count, 17);
    assert_eq!(metrics.bad_source_exits, 0);
    assert_eq!(metrics.bad_target_entries, 0);
    assert_eq!(metrics.endpoint_node_intrusions, 0);
    assert_eq!(metrics.endpoint_node_reentries, 0);
    assert_eq!(metrics.non_endpoint_node_hits, 0);
    // Upstream at 3726ccb: 76 bends, 14 crossings, 8506 layout units.
    // Leave headroom for unrelated, legitimate placement improvements.
    assert!(metrics.bends <= 30, "{metrics:?}");
    assert!(metrics.crossings <= 5, "{metrics:?}");
    assert!(metrics.path_length < 5200.0, "{metrics:?}");
}
