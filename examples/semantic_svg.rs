//! Bounded native graph profiles with libavoid; SVG to stdout.
use mermaid_rs_renderer::{
    LayoutConfig, Theme, config::FlowchartLayoutEngine, layout::compute_semantic_layout,
    parse_mermaid_strict, routing_backend::RoutingControl,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .ok_or("usage: semantic_svg diagram.mmd")?,
    )?;
    if source.len() > 65536 {
        return Err("source exceeds 64 KiB".into());
    }
    let graph = parse_mermaid_strict(&source)?.graph;
    let mut config = LayoutConfig::default();
    config.max_label_width_chars = 20;
    config.node_spacing = 70.;
    config.rank_spacing = 70.;
    config.flowchart.auto_spacing.enabled = false;
    if graph.subgraphs.is_empty() && graph.kind != mermaid_rs_renderer::ir::DiagramKind::State {
        config.flowchart.engine = FlowchartLayoutEngine::Dagre;
    } else {
        config.node_spacing = 50.;
        config.rank_spacing = 50.;
    }
    let (layout, diagnostics) = compute_semantic_layout(
        &graph,
        &Theme::modern(),
        &config,
        &RoutingControl {
            deadline: std::time::Instant::now() + std::time::Duration::from_secs(5),
            cancelled: &|| false,
        },
    )
    .map_err(|e| format!("{e}"))?;
    eprintln!("{}", diagnostics.join("; "));
    println!(
        "{}",
        mermaid_rs_renderer::render_svg(&layout, &Theme::modern(), &config)
    );
    Ok(())
}
