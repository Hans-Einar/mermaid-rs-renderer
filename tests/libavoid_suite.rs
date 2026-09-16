#![cfg(feature = "libavoid")]
use mermaid_rs_renderer::routing_backend::*;
use std::time::{Duration, Instant};
fn control() -> RoutingControl<'static> {
    RoutingControl {
        deadline: Instant::now() + Duration::from_secs(10),
        cancelled: &|| false,
    }
}
fn box_at(id: u32, x: f64, y: f64) -> Obstacle {
    Obstacle {
        id,
        polygon: vec![(x, y), (x + 60., y), (x + 60., y + 60.), (x, y + 60.)],
        ports: vec![
            Port {
                point: (x + 30., y),
                directions: 1,
            },
            Port {
                point: (x + 30., y + 60.),
                directions: 2,
            },
            Port {
                point: (x, y + 30.),
                directions: 4,
            },
            Port {
                point: (x + 60., y + 30.),
                directions: 8,
            },
        ],
    }
}
fn input() -> RoutingInput {
    RoutingInput {
        obstacles: vec![box_at(1, 0., 0.), box_at(2, 240., 0.), box_at(3, 120., 0.)],
        connections: vec![Connection {
            id: 37,
            source: 1,
            target: 2,
            source_directions: 15,
            target_directions: 15,
            source_port: None,
            target_port: None,
        }],
        clearance: 4.,
        separation: 8.,
        bend_cost: 20.,
        slide_ports: false,
    }
}
#[test]
fn avoids_obstacle_and_is_deterministic() {
    let i = input();
    let a = Libavoid.route(&i, &control()).unwrap();
    let b = Libavoid.route(&i, &control()).unwrap();
    assert_eq!(a.routes[0].id, 37);
    assert_eq!(a.routes[0].points, b.routes[0].points);
    for s in a.routes[0].points.windows(2) {
        assert!(s[0].0 == s[1].0 || s[0].1 == s[1].1);
        let m = ((s[0].0 + s[1].0) / 2., (s[0].1 + s[1].1) / 2.);
        assert!(!(m.0 > 120. && m.0 < 180. && m.1 > 0. && m.1 < 60.));
    }
}
#[test]
fn rejects_duplicate_ids_and_nonfinite_input() {
    let mut i = input();
    i.connections.push(i.connections[0].clone());
    assert!(matches!(
        Libavoid.route(&i, &control()),
        Err(RoutingError::InvalidInput(_))
    ));
    let mut i = input();
    i.obstacles[0].polygon[0].0 = f64::NAN;
    assert!(matches!(
        Libavoid.route(&i, &control()),
        Err(RoutingError::InvalidInput(_))
    ));
}
#[test]
fn cancellation_and_expired_deadline_publish_no_routes() {
    let c = RoutingControl {
        deadline: Instant::now() + Duration::from_secs(1),
        cancelled: &|| true,
    };
    assert!(matches!(
        Libavoid.route(&input(), &c),
        Err(RoutingError::Cancelled)
    ));
    let c = RoutingControl {
        deadline: Instant::now() - Duration::from_secs(1),
        cancelled: &|| false,
    };
    assert!(matches!(
        Libavoid.route(&input(), &c),
        Err(RoutingError::BudgetExceeded)
    ));
}
#[test]
fn parallel_opposite_and_self_loop_have_distinct_ids() {
    let mut i = input();
    i.obstacles.remove(2);
    for o in &mut i.obstacles {
        let existing = o.ports.clone();
        for p in existing {
            for delta in [-12., 12.] {
                let mut p = p.clone();
                if p.directions == UP || p.directions == DOWN {
                    p.point.0 += delta;
                } else {
                    p.point.1 += delta;
                }
                o.ports.push(p);
            }
        }
    }
    i.connections.extend([
        Connection {
            id: 38,
            source: 1,
            target: 2,
            source_directions: 15,
            target_directions: 15,
            source_port: None,
            target_port: None,
        },
        Connection {
            id: 39,
            source: 2,
            target: 1,
            source_directions: 15,
            target_directions: 15,
            source_port: None,
            target_port: None,
        },
        Connection {
            id: 40,
            source: 1,
            target: 1,
            source_directions: RIGHT,
            target_directions: DOWN,
            source_port: None,
            target_port: None,
        },
    ]);
    // Repeated transactions catch allocator-sensitive zero self-loop paths.
    for _ in 0..32 {
        let r = Libavoid.route(&i, &control()).unwrap();
        assert_eq!(r.routes.len(), 4);
        for route in &r.routes {
            assert!(route.points.len() >= 2);
        }
        assert_ne!(r.routes[0].points, r.routes[1].points);
    }
}

#[test]
fn visible_shape_ports_groups_and_cycles() {
    use mermaid_rs_renderer::layout::routed::{Engine, compute};
    use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict};
    for text in [
        "flowchart LR\n A[Rectangle] --> B(Rounded)\n B --> C{Diamond}\n C --> D((Circle))\n D --> A",
        "flowchart TD\n subgraph G[Group]\n A[One] --> B[Two]\n end\n C[Outside] --> A\n B --> C",
        "flowchart TD\n A[Self] --> A",
        "flowchart TD\n A(Rounded self) --> A",
        "flowchart TD\n A{Diamond self} --> A",
        "flowchart TD\n A((Circle self)) --> A",
        "flowchart LR\n A[One] --> B[Two]\n A --> B\n B --> A",
        "flowchart TD\n A-->D\n B-->D\n C-->D\n D-->E\n D-->F\n D-->G",
    ] {
        let graph = parse_mermaid_strict(text).unwrap().graph;
        let r = compute(
            &graph,
            &Theme::modern(),
            &LayoutConfig::default(),
            Engine::Libavoid,
            &control(),
        );
        assert!(r.is_ok(), "{text}: {r:?}");
    }
}
#[test]
fn traceability_labels_are_validated() {
    use mermaid_rs_renderer::layout::routed::{Engine, compute};
    use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict};
    let graph = parse_mermaid_strict(include_str!(
        "fixtures/flowchart/routing-review/traceability.mmd"
    ))
    .unwrap()
    .graph;
    let r = compute(
        &graph,
        &Theme::modern(),
        &LayoutConfig::default(),
        Engine::Libavoid,
        &control(),
    )
    .unwrap();
    assert_eq!(r.layout.edges.len(), 17);
    assert!(r.layout.edges.iter().all(|e| e.label_anchor.is_some()));
}

#[test]
fn cooperative_callback_abort_and_narrow_corridor() {
    use std::cell::Cell;
    let mut i = input();
    i.obstacles[2] = box_at(3, 120., -40.);
    i.obstacles.push(box_at(4, 120., 42.));
    let route = Libavoid.route(&i, &control()).unwrap();
    assert_eq!(route.routes.len(), 1);
    let checks = Cell::new(0);
    let cancelled = || {
        checks.set(checks.get() + 1);
        checks.get() > i.obstacles.len() + 3
    };
    let c = RoutingControl {
        deadline: Instant::now() + Duration::from_secs(10),
        cancelled: &cancelled,
    };
    assert!(matches!(
        Libavoid.route(&i, &c),
        Err(RoutingError::Cancelled)
    ));
    assert!(checks.get() > i.obstacles.len() + 3);
}
#[test]
fn rejects_degenerate_polygons_and_impossible_ports() {
    let mut i = input();
    i.obstacles[0].polygon[2] = i.obstacles[0].polygon[0];
    assert!(matches!(
        Libavoid.route(&i, &control()),
        Err(RoutingError::InvalidInput(_))
    ));
    let mut i = input();
    i.connections[0].source_port = Some((1., 1.));
    assert!(matches!(
        Libavoid.route(&i, &control()),
        Err(RoutingError::InvalidInput(_))
    ));
}
#[test]
fn placement_selection_is_independent_and_routes_repeat() {
    use mermaid_rs_renderer::config::FlowchartLayoutEngine;
    use mermaid_rs_renderer::layout::routed::{Engine, compute};
    use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict};
    let graph = parse_mermaid_strict("flowchart LR\n A-->B\n A-->C\n B-->D\n C-->D")
        .unwrap()
        .graph;
    for placement in [
        FlowchartLayoutEngine::Current,
        FlowchartLayoutEngine::Dagre,
        FlowchartLayoutEngine::Auto,
    ] {
        let mut config = LayoutConfig::default();
        config.flowchart.engine = placement;
        let a = compute(
            &graph,
            &Theme::modern(),
            &config,
            Engine::Libavoid,
            &control(),
        )
        .unwrap();
        let b = compute(
            &graph,
            &Theme::modern(),
            &config,
            Engine::Libavoid,
            &control(),
        )
        .unwrap();
        for (a, b) in a.layout.edges.iter().zip(&b.layout.edges) {
            assert_eq!(a.points, b.points);
        }
    }
}
#[test]
fn fixed_positions_and_text_are_preserved_and_quality_is_measured() {
    use mermaid_rs_renderer::layout::routed::{quality, route_positioned};
    use mermaid_rs_renderer::{LayoutConfig, Theme, compute_layout, parse_mermaid_strict};
    let graph = parse_mermaid_strict(include_str!(
        "fixtures/flowchart/routing-review/traceability.mmd"
    ))
    .unwrap()
    .graph;
    let old = compute_layout(&graph, &Theme::modern(), &LayoutConfig::default());
    let new = route_positioned(&old, &control()).unwrap().layout;
    let again = route_positioned(&old, &control()).unwrap().layout;
    for (id, n) in &old.nodes {
        let m = &new.nodes[id];
        assert_eq!((n.x, n.y, n.width, n.height), (m.x, m.y, m.width, m.height));
    }
    for ((a, b), c) in old.edges.iter().zip(&new.edges).zip(&again.edges) {
        assert_eq!(
            a.label.as_ref().map(|l| (l.width, l.height)),
            b.label.as_ref().map(|l| (l.width, l.height))
        );
        assert_eq!(b.points, c.points);
        assert_eq!(b.label_anchor, c.label_anchor);
    }
    let q = quality::measure(&new, 8.);
    assert_eq!(q.node_traversals, 0);
    assert_eq!(q.label_collisions, 0);
    assert!(q.length < quality::measure(&old, 8.).length);
}

#[test]
fn labelled_review_graphs_complete_with_bounded_spacing_retry() {
    use mermaid_rs_renderer::layout::routed::{Engine, compute, quality};
    use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict};
    for source in [
        include_str!("fixtures/flowchart/routing-review/service-map.mmd"),
        include_str!("fixtures/flowchart/routing-review/layer-delivery.mmd"),
    ] {
        let graph = parse_mermaid_strict(source).unwrap().graph;
        let result = compute(
            &graph,
            &Theme::modern(),
            &LayoutConfig::default(),
            Engine::Libavoid,
            &control(),
        )
        .unwrap();
        let q = quality::measure(&result.layout, 8.);
        assert_eq!(q.node_traversals, 0);
        assert_eq!(q.label_collisions, 0);
    }
}

#[test]
fn rejects_dense_work_before_native_nudging() {
    let mut i = input();
    i.connections = (0..257)
        .map(|id| {
            let mut e = i.connections[0].clone();
            e.id = id;
            e
        })
        .collect();
    let start = Instant::now();
    assert!(
        matches!(Libavoid.route(&i, &control()), Err(RoutingError::InvalidInput(s)) if s.contains("complexity limit"))
    );
    assert!(start.elapsed() < Duration::from_secs(1));
    let mut i = input();
    i.obstacles = (0..513)
        .map(|id| box_at(id, id as f64 * 100., 0.))
        .collect();
    assert!(
        matches!(Libavoid.route(&i, &control()), Err(RoutingError::InvalidInput(s)) if s.contains("complexity limit"))
    );
}
