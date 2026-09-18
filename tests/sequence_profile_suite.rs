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
        assert!(s.frames[0].y + s.frames[0].height < s.notes[0].y);
    } else {
        panic!("Missing sequence layout")
    }
}

#[test]
fn implicit_participants_are_not_actors() {
    let graph = parse_mermaid_strict("sequenceDiagram\nA->>B: Request")
        .unwrap()
        .graph;
    assert_eq!(
        graph.nodes["A"].shape,
        mermaid_rs_renderer::NodeShape::Rectangle
    );
}
#[test]
fn measured_multiline_labels_fit_fragment_scope() {
    use mermaid_rs_renderer::layout::{DiagramData, TextBlock, measurements};
    let graph=parse_mermaid_strict("sequenceDiagram\nparticipant A\nparticipant B\nA->>B: Begin\nloop retry\nA->>B: Long request\nB-->>A: Reply\nend").unwrap().graph;
    let mut labels = std::collections::HashMap::new();
    for key in ["A", "B", "Begin", "loop", "[retry]", "Reply"] {
        labels.insert(
            key.to_string(),
            TextBlock {
                lines: vec![key.into()],
                width: 60.0,
                height: 20.0,
            },
        );
    }
    labels.insert(
        "Long request".into(),
        TextBlock {
            lines: vec!["Long".into(), "request".into(), "with provenance".into()],
            width: 140.0,
            height: 60.0,
        },
    );
    let layout = measurements::layout(
        &graph,
        &Theme::modern(),
        &LayoutConfig::default(),
        labels,
        std::time::Duration::from_secs(2),
    );
    if let DiagramData::Sequence(s) = &layout.diagram {
        let frame = &s.frames[0];
        for label in &frame.section_labels {
            assert!(label.y - label.text.height / 2.0 >= frame.y);
        }
        let edge = &layout.edges[1];
        let (x, y) = edge.label_anchor.unwrap();
        let label = edge.label.as_ref().unwrap();
        assert!(
            x - label.width / 2.0 >= frame.x && x + label.width / 2.0 <= frame.x + frame.width,
            "message escaped horizontally"
        );
        assert!(
            y - label.height / 2.0 >= frame.y && y + label.height / 2.0 <= frame.y + frame.height,
            "message escaped vertically"
        );
    } else {
        panic!("Missing sequence")
    }
}
#[test]
fn explicit_event_order_preserves_nested_scope() {
    use mermaid_rs_renderer::ir::{SequenceEvent as E, SequenceFrameKind as F};
    use mermaid_rs_renderer::layout::DiagramData;
    let mut graph = parse_mermaid_strict(
        "sequenceDiagram\nparticipant A\nparticipant B\nA-)B: Request\nB--)A: Reply",
    )
    .unwrap()
    .graph;
    graph.sequence_events = vec![
        E::Start(F::Alt, "outer".into()),
        E::Start(F::Opt, "inner".into()),
        E::Message(0),
        E::End,
        E::Branch("other".into()),
        E::Message(1),
        E::End,
    ];
    let layout = compute_layout(&graph, &Theme::modern(), &LayoutConfig::default());
    let DiagramData::Sequence(s) = &layout.diagram else {
        panic!("sequence")
    };
    assert_eq!(s.frames.len(), 2);
    assert!(
        s.frames[1].y > s.frames[0].y
            && s.frames[1].y + s.frames[1].height < s.frames[0].dividers[0]
    );
    assert!(layout.edges[0].points[0].1 < layout.edges[1].points[0].1);
    let svg = render_svg(&layout, &Theme::modern(), &LayoutConfig::default());
    assert_eq!(svg.matches("marker-end=\"url(#arrow-async-").count(), 2);
}
