use super::*;
use crate::layout::{
    geometry::{EdgeSide, shape_polygon_points},
    routing::anchor_point_for_node,
};
use std::collections::BTreeMap;
pub(super) fn rectangle(x: f64, y: f64, w: f64, h: f64) -> Vec<Point> {
    vec![(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
}
pub(super) fn input(layout: &Layout) -> Result<RoutingInput, RoutingError> {
    let mut obstacles = Vec::new();
    let mut ids = BTreeMap::new();
    for (id, n) in &layout.nodes {
        if n.hidden {
            continue;
        }
        let degree = layout
            .edges
            .iter()
            .filter(|e| e.from == *id || e.to == *id)
            .count();
        let count = (degree + 1).clamp(3, 15);
        let mut ports = Vec::new();
        for (side, dir, len) in [
            (EdgeSide::Top, 1, n.width),
            (EdgeSide::Bottom, 2, n.width),
            (EdgeSide::Left, 4, n.height),
            (EdgeSide::Right, 8, n.height),
        ] {
            for i in 0..count {
                let offset = (i as f32 / (count - 1) as f32 - 0.5) * (len - 16.).max(0.) * 0.8;
                let p = anchor_point_for_node(n, side, offset);
                let point = (p.0 as f64, p.1 as f64);
                // Small pseudostates have no tangential port span. Do not
                // register multiple exclusive pins at the same location.
                if !ports
                    .iter()
                    .any(|port: &Port| port.point == point && port.directions == dir)
                {
                    ports.push(Port {
                        point,
                        directions: dir,
                    });
                }
            }
        }
        let oid = obstacles.len() as u32 + 1;
        ids.insert(id.clone(), oid);
        obstacles.push(Obstacle {
            id: oid,
            polygon: rectangle(n.x as f64, n.y as f64, n.width as f64, n.height as f64),
            ports,
        });
    }
    // Group interiors stay traversable. Their measured title strips do not.
    for g in &layout.subgraphs {
        if g.label.is_empty() {
            continue;
        }
        obstacles.push(Obstacle {
            id: obstacles.len() as u32 + 1,
            polygon: rectangle(
                (g.x + g.width / 2. - g.label_block.width / 2.) as f64,
                (g.y + 3.) as f64,
                g.label_block.width as f64,
                g.label_block.height as f64 + 4.,
            ),
            ports: vec![],
        });
    }
    let mut connections = Vec::new();
    for (i, e) in layout.edges.iter().enumerate() {
        let source = *ids.get(&e.from).ok_or_else(|| {
            RoutingError::InvalidInput(format!("unsupported hidden/group endpoint {}", e.from))
        })?;
        let target = *ids.get(&e.to).ok_or_else(|| {
            RoutingError::InvalidInput(format!("unsupported hidden/group endpoint {}", e.to))
        })?;
        connections.push(Connection {
            id: i as u32,
            source,
            target,
            source_directions: if e.from == e.to {
                RIGHT
            } else {
                ALL_DIRECTIONS
            },
            target_directions: if e.from == e.to { DOWN } else { ALL_DIRECTIONS },
            source_port: None,
            target_port: None,
        });
    }
    Ok(RoutingInput {
        obstacles,
        connections,
        clearance: 4.,
        separation: 8.,
        bend_cost: 20.,
        slide_ports: layout
            .nodes
            .values()
            .filter(|n| !n.hidden)
            .all(|n| n.shape == crate::ir::NodeShape::Rectangle),
    })
}
pub(super) fn validate(layout: &Layout) -> Result<(), RoutingError> {
    for (idx, e) in layout.edges.iter().enumerate() {
        if e.points.len() < 2
            || e.points
                .iter()
                .any(|p| !p.0.is_finite() || !p.1.is_finite())
        {
            return Err(RoutingError::Backend(format!("invalid points: edge {idx}")));
        }
        for pair in e.points.windows(2) {
            if (pair[0].0 - pair[1].0).abs() > 0.01 && (pair[0].1 - pair[1].1).abs() > 0.01 {
                return Err(RoutingError::Backend(format!("non-orthogonal edge {idx}")));
            }
            for (id, n) in &layout.nodes {
                if n.hidden {
                    continue;
                }
                if interior(pair[0], pair[1], n) {
                    return Err(RoutingError::Backend(format!(
                        "edge {idx} traverses node {id}"
                    )));
                }
            }
        }
    }
    Ok(())
}

/// Analytic segment/interior test. Rounded rectangles use conservative boxes.
pub(super) fn interior(a: (f32, f32), b: (f32, f32), n: &crate::layout::NodeLayout) -> bool {
    use crate::ir::NodeShape;
    if matches!(n.shape, NodeShape::Circle | NodeShape::DoubleCircle) {
        let cx = (n.x + n.width / 2.) as f64;
        let cy = (n.y + n.height / 2.) as f64;
        let a = (
            (a.0 as f64 - cx) / (n.width as f64 / 2.),
            (a.1 as f64 - cy) / (n.height as f64 / 2.),
        );
        let b = (
            (b.0 as f64 - cx) / (n.width as f64 / 2.),
            (b.1 as f64 - cy) / (n.height as f64 / 2.),
        );
        let d = (b.0 - a.0, b.1 - a.1);
        let den = d.0 * d.0 + d.1 * d.1;
        let t = if den > 0. {
            (-(a.0 * d.0 + a.1 * d.1) / den).clamp(0., 1.)
        } else {
            0.
        };
        return (a.0 + t * d.0).powi(2) + (a.1 + t * d.1).powi(2) < 1. - 1e-5;
    }
    let Some(poly) = shape_polygon_points(n) else {
        return false;
    };
    let area: f64 = (0..poly.len())
        .map(|i| {
            let p = poly[i];
            let q = poly[(i + 1) % poly.len()];
            (p.0 * q.1 - p.1 * q.0) as f64
        })
        .sum();
    let mut lo: f64 = 0.;
    let mut hi: f64 = 1.;
    for i in 0..poly.len() {
        let p = poly[i];
        let q = poly[(i + 1) % poly.len()];
        let cross = |r: (f32, f32)| {
            ((q.0 - p.0) as f64 * (r.1 - p.1) as f64 - (q.1 - p.1) as f64 * (r.0 - p.0) as f64)
                * area.signum()
        };
        let v = cross(a) - (q.0 - p.0).hypot(q.1 - p.1) as f64 * 0.01;
        let d = cross(b) - cross(a);
        if d.abs() < 1e-9 {
            if v <= 0. {
                return false;
            }
        } else if d > 0. {
            lo = lo.max(-v / d);
        } else {
            hi = hi.min(-v / d);
        }
    }
    hi > lo + 1e-7
}
