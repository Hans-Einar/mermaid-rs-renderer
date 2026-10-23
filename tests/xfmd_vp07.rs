#![cfg(feature = "libavoid")]
use mermaid_rs_renderer::layout::{TextBlock, measurements, routed};
use mermaid_rs_renderer::routing_backend::RoutingControl;
use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
#[test]
fn xfmd_vp07_preserves_all_nodes_and_orthogonal_edges() {
    let graph = parse_mermaid_strict(include_str!("fixtures/xfmd/vp07.mmd"))
        .unwrap()
        .graph;
    let values: HashMap<String, [f32; 2]> =
        serde_json::from_str(include_str!("fixtures/xfmd/vp07-metrics.json")).unwrap();
    let labels = values
        .into_iter()
        .map(|(label, size)| {
            (
                label.clone(),
                TextBlock {
                    lines: vec![label],
                    width: size[0],
                    height: size[1],
                },
            )
        })
        .collect();
    let mut theme = Theme::modern();
    theme.font_size = 16.;
    theme.font_family = "DejaVu Sans".into();
    let budget = Duration::from_secs(2);
    let result = measurements::with_measurements(labels, budget, || {
        routed::compute(
            &graph,
            &theme,
            &LayoutConfig::default(),
            routed::Engine::Libavoid,
            &RoutingControl {
                deadline: Instant::now() + budget,
                cancelled: &|| false,
            },
        )
    })
    .unwrap();
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.contains("one spacing retry"))
    );
    assert_eq!(result.layout.nodes.len(), 12);
    assert_eq!(result.layout.edges.len(), 18);
    for edge in &result.layout.edges {
        assert!(edge.points.len() >= 2);
        for p in edge.points.windows(2) {
            assert!((p[0].0 - p[1].0).abs() < 0.01 || (p[0].1 - p[1].1).abs() < 0.01);
        }
    }
    eprintln!("{:?}", result.diagnostics);
}
