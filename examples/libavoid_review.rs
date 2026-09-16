use mermaid_rs_renderer::layout::{
    flowchart_quality_metrics,
    routed::{self, Engine},
};
use mermaid_rs_renderer::routing_backend::RoutingControl;
use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict, render_svg};
use std::time::{Duration, Instant};
fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    anyhow::ensure!(
        args.len() >= 3,
        "usage: libavoid_review INPUT OUTPUT_PREFIX [fixed]"
    );
    let graph = parse_mermaid_strict(&std::fs::read_to_string(&args[1])?)?.graph;
    let theme = Theme::modern();
    let config = LayoutConfig::default();
    let control = RoutingControl {
        deadline: Instant::now() + Duration::from_secs(60),
        cancelled: &|| false,
    };
    let result = if args.get(3).is_some_and(|s| s == "fixed") {
        let legacy = mermaid_rs_renderer::compute_layout(&graph, &theme, &config);
        std::fs::write(
            format!("{}-legacy.svg", args[2]),
            render_svg(&legacy, &theme, &config),
        )?;
        println!("legacy {:?}", flowchart_quality_metrics(&legacy));
        routed::route_positioned(&legacy, &control)?
    } else {
        routed::compute(&graph, &theme, &config, Engine::Libavoid, &control)?
    };
    println!(
        "libavoid {:?} routing={:?} total={:?} {:?}",
        flowchart_quality_metrics(&result.layout),
        result.routing_time,
        result.total_time,
        result.diagnostics
    );
    std::fs::write(
        format!("{}.svg", args[2]),
        render_svg(&result.layout, &theme, &config),
    )?;
    mermaid_rs_renderer::layout_dump::write_layout_dump(
        std::path::Path::new(&format!("{}.json", args[2])),
        &result.layout,
        &graph,
    )?;
    Ok(())
}
