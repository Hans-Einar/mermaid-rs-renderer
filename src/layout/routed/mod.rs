//! Explicit routing selection independent of the configured node placement engine.
mod geometry;
mod labels;
pub mod quality;
use super::Layout;
use crate::routing_backend::*;
use crate::{Graph, LayoutConfig, Theme, ir::DiagramKind};
use std::time::Instant;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Engine {
    Legacy,
    Libavoid,
}
#[derive(Debug)]
pub struct RoutedLayout {
    pub layout: Layout,
    pub engine: Engine,
    pub diagnostics: Vec<String>,
    pub routing_time: std::time::Duration,
    pub total_time: std::time::Duration,
}
/// Legacy is explicit. No automatic fallback hides a backend error.
pub fn compute(
    graph: &Graph,
    theme: &Theme,
    config: &LayoutConfig,
    engine: Engine,
    control: &RoutingControl<'_>,
) -> Result<RoutedLayout, RoutingError> {
    control.check()?;
    let start = Instant::now();
    if engine == Engine::Legacy {
        let (layout, metrics) = super::compute_layout_with_metrics(graph, theme, config);
        control.check()?;
        return Ok(RoutedLayout {
            layout,
            engine,
            diagnostics: vec![
                "Legacy routing: cancellation checked before/after layout only".into(),
            ],
            routing_time: std::time::Duration::from_micros(
                (metrics.edge_routing_us + metrics.port_assignment_us) as u64,
            ),
            total_time: start.elapsed(),
        });
    }
    if graph.kind != DiagramKind::Flowchart {
        return Err(RoutingError::InvalidInput(
            "alternative routing supports flowcharts only".into(),
        ));
    }
    let graph = super::normalize_graph_for_layout(graph);
    let layout = super::compute_flowchart_layout(&graph, theme, config, None, false);
    control.check()?;
    let attempt = Instant::now();
    let mut result = match route_positioned(&layout, control) {
        Ok(result) => result,
        Err(RoutingError::NoSpace(reason)) => {
            // One explicit placement retry for label space. Never applied to
            // route_positioned, which guarantees frozen node geometry.
            let failed = attempt.elapsed();
            control.check()?;
            let mut expanded = config.clone();
            expanded.flowchart.auto_spacing.enabled = false;
            expanded.node_spacing = config.node_spacing.max(50.) * 2. + 24.;
            expanded.rank_spacing = config.rank_spacing.max(50.) * 2. + 24.;
            let layout = super::compute_flowchart_layout(&graph, theme, &expanded, None, false);
            control.check()?;
            let mut r = route_positioned(&layout, control)?;
            r.diagnostics.insert(0,format!("one spacing retry after {reason}; failed routing/labels attempt {failed:?}, included in total time"));
            r
        }
        Err(error) => return Err(error),
    };
    // Only uniform translation/bounds adjustment after routing; never Legacy
    // route repair, label nudge, coordinate scaling or aspect folding.
    super::finalize_graph_label_bounds(&mut result.layout, config);
    geometry::validate(&result.layout)?;
    result.total_time = start.elapsed();
    Ok(result)
}
/// Route an already placed/measured graph without moving its nodes. Used for
/// identical-position A/B comparisons and by the fresh-placement path above.
pub fn route_positioned(
    layout: &Layout,
    control: &RoutingControl<'_>,
) -> Result<RoutedLayout, RoutingError> {
    control.check()?;
    #[cfg(not(feature = "libavoid"))]
    {
        let _ = layout;
        Err(RoutingError::Unavailable)
    }
    #[cfg(feature = "libavoid")]
    {
        let start = Instant::now();
        let mut layout = layout.clone();
        let mut input = geometry::input(&layout)?;
        let mut diagnostics = vec![];
        let mut routing_time = std::time::Duration::ZERO;
        let mut completed = false;
        for pass in 0..3 {
            control.check()?;
            let output = Libavoid.route(&input, control)?;
            diagnostics.push(format!(
                "libavoid transaction {}: {:?}",
                pass + 1,
                output.elapsed
            ));
            routing_time += output.elapsed;
            for route in output.routes {
                if input.slide_ports {
                    let c = &input.connections[route.id as usize];
                    for (id, p) in [(c.source, route.source_port), (c.target, route.target_port)] {
                        let o = input.obstacles.iter_mut().find(|o| o.id == id).unwrap();
                        let xs: Vec<_> = o.polygon.iter().map(|p| p.0).collect();
                        let ys: Vec<_> = o.polygon.iter().map(|p| p.1).collect();
                        let dir = if (p.1 - ys.iter().copied().fold(f64::INFINITY, f64::min)).abs()
                            < 0.01
                        {
                            UP
                        } else if (p.1 - ys.iter().copied().fold(f64::NEG_INFINITY, f64::max)).abs()
                            < 0.01
                        {
                            DOWN
                        } else if (p.0 - xs.iter().copied().fold(f64::INFINITY, f64::min)).abs()
                            < 0.01
                        {
                            LEFT
                        } else {
                            RIGHT
                        };
                        o.ports.push(Port {
                            point: p,
                            directions: dir,
                        });
                    }
                }
                let connection = &mut input.connections[route.id as usize];
                connection.source_port = Some(route.source_port);
                connection.target_port = Some(route.target_port);
                let edge = layout
                    .edges
                    .get_mut(route.id as usize)
                    .ok_or_else(|| RoutingError::Backend("unknown edge ID".into()))?;
                edge.points = route
                    .points
                    .iter()
                    .map(|p| (p.0 as f32, p.1 as f32))
                    .collect();
            }
            input.slide_ports = false;
            geometry::validate(&layout)?;
            if pass == 0 {
                labels::place(&mut layout)?;
                if labels::validate(&layout).is_ok() {
                    completed = true;
                    break;
                }
                input
                    .obstacles
                    .extend(labels::obstacles(&layout, input.obstacles.len() as u32 + 1));
                if layout.edges.iter().all(|e| e.label.is_none()) {
                    completed = true;
                    break;
                }
            } else if labels::validate(&layout).is_ok() {
                completed = true;
                break;
            } else if pass == 1 {
                labels::place(&mut layout)?;
                // Keep selected sliding pins and remove only label obstacles.
                let node_count = geometry::input(&layout)?.obstacles.len();
                input.obstacles.truncate(node_count);
                input
                    .obstacles
                    .extend(labels::obstacles(&layout, input.obstacles.len() as u32 + 1));
            }
        }
        if !completed {
            labels::validate(&layout)?;
        }
        control.check()?;
        let quality = quality::measure(&layout, input.separation);
        if quality.parallel_overlaps > 0 || quality.close_parallel_segments > 0 {
            diagnostics.push(format!("constrained parallel separation: {quality:?}"));
        }
        Ok(RoutedLayout {
            layout,
            engine: Engine::Libavoid,
            diagnostics,
            routing_time,
            total_time: start.elapsed(),
        })
    }
}
