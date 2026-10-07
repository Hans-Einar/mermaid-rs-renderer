use mermaid_rs_renderer::{
    Graph, LayoutConfig, Theme, ir::*, layout::compute_semantic_layout,
    routing_backend::RoutingControl,
};
use std::time::{Duration, Instant};
fn graph(boundary: bool) -> Graph {
    let mut g = Graph::new();
    g.kind = DiagramKind::UseCase;
    g.direction = Direction::LeftRight;
    for (id, label, shape) in [
        ("Author", "Author", NodeShape::UseCaseActor),
        ("Browse", "Browse project", NodeShape::Ellipse),
        ("Read", "Read source", NodeShape::Ellipse),
        ("Unused", "Independent case", NodeShape::Ellipse),
    ] {
        g.node_order.insert(id.into(), g.nodes.len());
        g.nodes.insert(
            id.into(),
            Node {
                id: id.into(),
                label: label.into(),
                shape,
                value: None,
                icon: None,
            },
        );
    }
    for (from, to, label, dotted) in [
        ("Author", "Browse", "", false),
        ("Browse", "Read", "«include»", true),
    ] {
        g.edges.push(Edge {
            from: from.into(),
            to: to.into(),
            label: (!label.is_empty()).then(|| label.into()),
            start_label: None,
            end_label: None,
            directed: true,
            arrow_start: false,
            arrow_end: true,
            arrow_start_kind: None,
            arrow_end_kind: Some(EdgeArrowhead::ClassDependency),
            start_decoration: None,
            end_decoration: None,
            style: if dotted {
                EdgeStyle::Dotted
            } else {
                EdgeStyle::Solid
            },
        });
    }
    if boundary {
        g.subgraphs.push(Subgraph {
            id: Some("Viewer".into()),
            label: "Viewer".into(),
            nodes: vec!["Browse".into(), "Read".into(), "Unused".into()],
            direction: None,
            icon: None,
        });
    }
    g
}
#[test]
fn native_usecase_glyphs_relations_and_isolated_elements() {
    for bounded in [false, true] {
        let g = graph(bounded);
        let theme = Theme::modern();
        let config = LayoutConfig::default();
        let (layout, _) = compute_semantic_layout(
            &g,
            &theme,
            &config,
            &RoutingControl {
                deadline: Instant::now() + Duration::from_secs(5),
                cancelled: &|| false,
            },
        )
        .unwrap();
        assert_eq!(
            layout
                .nodes
                .values()
                .filter(|n| !n.hidden && n.anchor_subgraph.is_none())
                .count(),
            4
        );
        assert_eq!(layout.edges.len(), 2);
        let actor = &layout.nodes["Author"];
        assert!(actor.height > actor.label.height + 50.);
        let uc = &layout.nodes["Browse"];
        assert!(uc.width >= uc.label.width * std::f32::consts::SQRT_2);
        assert_eq!(layout.edges[1].style, EdgeStyle::Dotted);
        let svg = mermaid_rs_renderer::render_svg(&layout, &theme, &config);
        assert!(svg.contains("usecase-actor"));
        assert_eq!(svg.matches("usecase-ellipse").count(), 3);
        for label in ["Author", "Browse", "Independent", "include"] {
            assert!(svg.contains(label), "{label}");
        }
        for rest in svg.split("url(#").skip(1) {
            let marker = rest.split(')').next().unwrap();
            assert!(
                svg.contains(&format!("id=\"{marker}\"")),
                "undefined marker {marker}"
            );
        }
        if bounded {
            assert!(svg.contains("Viewer"));
        }
    }
}
