#![cfg(feature = "libavoid")]
use mermaid_rs_renderer::layout::{TextBlock, measurements, routed};
use mermaid_rs_renderer::routing_backend::RoutingControl;
use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
#[test]
fn xfmd_document_references_preserves_routes_and_caption_attachments() {
    let parsed = parse_mermaid_strict(include_str!("fixtures/xfmd/document-references.mmd"))
        .unwrap()
        .graph;
    // Reproduce XFMD's typed-contract round trip, including contiguous node
    // order and absent optional arrow decorations, not only direct parser layout.
    let mut graph = mermaid_rs_renderer::Graph::new();
    graph.direction = parsed.direction;
    let mut nodes: Vec<_> = parsed.nodes.values().cloned().collect();
    nodes.sort_by_key(|n| {
        (
            parsed.node_order.get(&n.id).copied().unwrap_or(usize::MAX),
            n.id.clone(),
        )
    });
    for (i, node) in nodes.into_iter().enumerate() {
        graph.node_order.insert(node.id.clone(), i);
        graph.nodes.insert(node.id.clone(), node);
    }
    graph.edges = parsed.edges;
    for e in &mut graph.edges {
        e.arrow_start_kind = None;
        e.arrow_end_kind = None;
        e.start_decoration = None;
        e.end_decoration = None;
    }
    let values: HashMap<String, [f32; 2]> = serde_json::from_str(include_str!(
        "fixtures/xfmd/document-references-metrics.json"
    ))
    .unwrap();
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
    let quality = routed::quality::measure(&result.layout, 8.);
    assert_eq!(quality.node_traversals, 0);
    assert_eq!(quality.label_collisions, 0);
    assert_eq!(quality.attachment_intrusions, 0);
    assert_eq!(result.layout.nodes.len(), 12);
    assert_eq!(result.layout.edges.len(), 15);
    for edge in &result.layout.edges {
        assert!(edge.points.len() >= 2);
        for p in edge.points.windows(2) {
            assert!((p[0].0 - p[1].0).abs() < 0.01 || (p[0].1 - p[1].1).abs() < 0.01);
        }
    }
    eprintln!("{:?}", result.diagnostics);
}
