use mermaid_rs_renderer::*;
#[test]
fn packet_uses_bits_and_splits_rows() {
    let graph = parse_mermaid_strict("packet\n0-7: \"Type\"\n8-31: \"Revision\"\n+40: \"Payload\"")
        .unwrap()
        .graph;
    let layout = compute_layout(&graph, &Theme::modern(), &LayoutConfig::default());
    assert!(layout.edges.is_empty());
    assert!(
        (layout.nodes["packet_1_0"].width / layout.nodes["packet_0_0"].width - 3.).abs() < 0.01
    );
    assert!(layout.nodes.contains_key("packet_2_2"));
    let svg = render_svg(&layout, &Theme::modern(), &LayoutConfig::default());
    assert!(svg.contains("Payload"));
}
#[test]
fn packet_rejects_overlapping_fields() {
    assert!(parse_mermaid_strict("packet\n0-7: \"a\"\n4-9: \"b\"").is_err());
}
#[test]
fn block_connectors_end_at_boundaries() {
    let graph = parse_mermaid_strict("block-beta\ncolumns 1\nA[A]\nB[B]\nA --> B")
        .unwrap()
        .graph;
    let l = compute_layout(&graph, &Theme::modern(), &LayoutConfig::default());
    let e = &l.edges[0];
    let b = &l.nodes["B"];
    assert!((e.points.last().unwrap().1 - b.y).abs() < 0.01);
}
