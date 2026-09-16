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
