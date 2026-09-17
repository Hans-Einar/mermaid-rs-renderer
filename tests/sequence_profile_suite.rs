use mermaid_rs_renderer::{LayoutConfig, Theme, compute_layout, parse_mermaid_strict, render_svg};
#[test]
fn actors_and_participants_remain_distinct() {
    let graph=parse_mermaid_strict("sequenceDiagram\nactor A as Reader\nparticipant B as Service\nA->>B: Request\nB-->>A: Reply").unwrap().graph;
    assert_ne!(graph.nodes["A"].shape, graph.nodes["B"].shape);
    let theme = Theme::modern();
    let config = LayoutConfig::default();
    let layout = compute_layout(&graph, &theme, &config);
    let svg = render_svg(&layout, &theme, &config);
    assert_eq!(svg.matches("class=\"sequence-actor\"").count(), 2);
    assert!(svg.contains("Reader") && svg.contains("Service"));
}
#[test]
fn sequence_checks_expired_measurement_budget() {
    let graph = parse_mermaid_strict("sequenceDiagram\nparticipant A\nA->>A: x")
        .unwrap()
        .graph;
    assert!(
        std::panic::catch_unwind(|| mermaid_rs_renderer::layout::measurements::layout(
            &graph,
            &Theme::modern(),
            &LayoutConfig::default(),
            std::collections::HashMap::new(),
            std::time::Duration::ZERO
        ))
        .is_err()
    );
}
#[test]
fn note_after_fragment_is_outside_its_bounds() {
    let graph=parse_mermaid_strict("sequenceDiagram\nparticipant A\nparticipant B\nA->>B: Begin\nalt valid\nB-->>A: accepted\nelse invalid\nB-->>A: rejected\nend\nNote over A,B: Outside").unwrap().graph;
    let layout = compute_layout(&graph, &Theme::modern(), &LayoutConfig::default());
    if let mermaid_rs_renderer::layout::DiagramData::Sequence(s) = layout.diagram {
        assert!(s.frames[0].y + s.frames[0].height < s.notes[0].y + s.notes[0].height);
    } else {
        panic!("Missing sequence layout")
    }
}
