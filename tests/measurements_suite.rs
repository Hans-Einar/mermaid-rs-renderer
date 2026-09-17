use mermaid_rs_renderer::layout::{TextBlock, measurements};
use std::{collections::HashMap, time::Duration};
#[test]
fn measurement_scope_restores_state_after_unwind_and_nesting() {
    let labels = HashMap::from([(
        "A".into(),
        TextBlock {
            lines: vec!["A".into()],
            width: 123.,
            height: 20.,
        },
    )]);
    measurements::with_measurements(labels.clone(), Duration::from_secs(1), || {
        assert_eq!(measurements::lookup("A").unwrap().width, 123.);
        let result = std::panic::catch_unwind(|| {
            measurements::with_measurements(HashMap::new(), Duration::ZERO, || {
                for _ in 0..64 {
                    measurements::checkpoint();
                }
            })
        });
        assert!(result.is_err());
        assert_eq!(measurements::lookup("A").unwrap().width, 123.);
    });
    assert!(measurements::lookup("A").is_none());
    for _ in 0..64 {
        measurements::checkpoint();
    }
}
#[cfg(feature = "libavoid")]
#[test]
fn external_measurements_reach_alternative_routing_without_reparsing() {
    use mermaid_rs_renderer::layout::routed::{Engine, compute};
    use mermaid_rs_renderer::routing_backend::RoutingControl;
    use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict};
    let graph = parse_mermaid_strict("flowchart LR\n A[Measured] --> B[Other]")
        .unwrap()
        .graph;
    let labels = [("Measured", 123.), ("Other", 87.), ("", 0.)]
        .into_iter()
        .map(|(s, w)| {
            (
                s.into(),
                TextBlock {
                    lines: vec![s.into()],
                    width: w,
                    height: 20.,
                },
            )
        })
        .collect();
    let c = RoutingControl {
        deadline: std::time::Instant::now() + Duration::from_secs(1),
        cancelled: &|| false,
    };
    let r = measurements::with_measurements(labels, Duration::from_secs(1), || {
        compute(
            &graph,
            &Theme::modern(),
            &LayoutConfig::default(),
            Engine::Libavoid,
            &c,
        )
    })
    .unwrap();
    assert_eq!(r.layout.nodes["A"].label.width, 123.);
    assert_eq!(r.layout.nodes["B"].label.width, 87.);
}

#[cfg(feature = "libavoid")]
#[test]
fn traceability_with_ubuntu_pango_measurements() {
    use mermaid_rs_renderer::layout::routed::{Engine, compute, quality};
    use mermaid_rs_renderer::routing_backend::RoutingControl;
    use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict};
    let graph = parse_mermaid_strict(include_str!(
        "fixtures/flowchart/routing-review/traceability.mmd"
    ))
    .unwrap()
    .graph;
    let values: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/flowchart/routing-review/traceability-ubuntu-pango.json"
    ))
    .unwrap();
    let labels = values
        .as_object()
        .unwrap()
        .iter()
        .map(|(s, v)| {
            (
                s.clone(),
                TextBlock {
                    lines: vec![s.clone()],
                    width: v["width"].as_f64().unwrap() as f32,
                    height: v["height"].as_f64().unwrap() as f32,
                },
            )
        })
        .collect();
    let mut theme = Theme::modern();
    theme.font_size = 16.;
    theme.font_family = "DejaVu Sans".into();
    let c = RoutingControl {
        deadline: std::time::Instant::now() + Duration::from_secs(5),
        cancelled: &|| false,
    };
    let r = measurements::with_measurements(labels, Duration::from_secs(5), || {
        compute(
            &graph,
            &theme,
            &LayoutConfig::default(),
            Engine::Libavoid,
            &c,
        )
    })
    .unwrap();
    assert_eq!(quality::measure(&r.layout, 8.).label_collisions, 0);
}

#[cfg(feature = "libavoid")]
#[test]
fn wrapped_layer_delivery_keeps_valid_ports_and_attachment() {
    use mermaid_rs_renderer::layout::routed::{Engine, compute, quality};
    use mermaid_rs_renderer::routing_backend::RoutingControl;
    use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict};
    let graph = parse_mermaid_strict(include_str!(
        "fixtures/flowchart/routing-review/layer-delivery.mmd"
    ))
    .unwrap()
    .graph;
    let values: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/flowchart/routing-review/layer-delivery-xfmd-wrapped.json"
    ))
    .unwrap();
    let labels = values
        .as_object()
        .unwrap()
        .iter()
        .map(|(s, v)| {
            (
                s.clone(),
                TextBlock {
                    lines: v["lines"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|s| s.as_str().unwrap().to_owned())
                        .collect(),
                    width: v["width"].as_f64().unwrap() as f32,
                    height: v["height"].as_f64().unwrap() as f32,
                },
            )
        })
        .collect();
    let mut theme = Theme::modern();
    theme.font_size = 16.;
    theme.font_family = "DejaVu Sans".into();
    let c = RoutingControl {
        deadline: std::time::Instant::now() + Duration::from_secs(10),
        cancelled: &|| false,
    };
    let r = measurements::with_measurements(labels, Duration::from_secs(10), || {
        compute(
            &graph,
            &theme,
            &LayoutConfig::default(),
            Engine::Libavoid,
            &c,
        )
    })
    .unwrap();
    let q = quality::measure(&r.layout, 8.);
    assert_eq!(q.attachment_intrusions, 0);
    assert_eq!(q.label_collisions, 0);
    assert_eq!(q.node_traversals, 0);
}
