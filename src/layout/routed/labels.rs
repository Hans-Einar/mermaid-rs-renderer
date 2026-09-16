use super::geometry::rectangle;
use super::*;
type Rect = (f64, f64, f64, f64);
fn overlap(a: Rect, b: Rect) -> bool {
    a.0 < b.0 + b.2 && b.0 < a.0 + a.2 && a.1 < b.1 + b.3 && b.1 < a.1 + a.3
}
fn rect(p: (f32, f32), w: f32, h: f32) -> Rect {
    (
        (p.0 - w / 2.) as f64 - 4.,
        (p.1 - h / 2.) as f64 - 4.,
        w as f64 + 8.,
        h as f64 + 8.,
    )
}
fn node_rects(layout: &Layout) -> Vec<Rect> {
    let mut r: Vec<_> = layout
        .nodes
        .values()
        .filter(|n| !n.hidden)
        .map(|n| {
            (
                n.x as f64 - 12.,
                n.y as f64 - 12.,
                n.width as f64 + 24.,
                n.height as f64 + 24.,
            )
        })
        .collect();
    r.extend(
        layout
            .subgraphs
            .iter()
            .filter(|g| !g.label.is_empty())
            .map(|g| {
                (
                    (g.x + g.width / 2. - g.label_block.width / 2.) as f64,
                    g.y as f64,
                    g.label_block.width as f64,
                    g.label_block.height as f64 + 8.,
                )
            }),
    );
    r
}
fn hit(points: &[(f32, f32)], r: Rect) -> bool {
    points.windows(2).any(|s| {
        let (a, b) = (
            (s[0].0 as f64, s[0].1 as f64),
            (s[1].0 as f64, s[1].1 as f64),
        );
        if (a.0 - b.0).abs() < 0.01 {
            a.0 > r.0 && a.0 < r.0 + r.2 && a.1.min(b.1) < r.1 + r.3 && a.1.max(b.1) > r.1
        } else {
            a.1 > r.1 && a.1 < r.1 + r.3 && a.0.min(b.0) < r.0 + r.2 && a.0.max(b.0) > r.0
        }
    })
}
fn near(points: &[(f32, f32)], a: (f32, f32), w: f32, h: f32) -> bool {
    points.windows(2).any(|s| {
        let x = a.0.clamp(s[0].0.min(s[1].0), s[0].0.max(s[1].0));
        let y = a.1.clamp(s[0].1.min(s[1].1), s[0].1.max(s[1].1));
        ((x - a.0).abs() - w / 2.).max(0.) + ((y - a.1).abs() - h / 2.).max(0.) < 48.
    })
}
pub(super) fn place(layout: &mut Layout) -> Result<(), RoutingError> {
    let mut occupied = node_rects(layout);
    let mut order: Vec<_> = (0..layout.edges.len()).collect();
    order.sort_by(|&a, &b| {
        let area = |i: usize| {
            layout.edges[i]
                .label
                .as_ref()
                .map_or(0., |l| l.width * l.height)
        };
        area(b).total_cmp(&area(a)).then(a.cmp(&b))
    });
    for idx in order {
        let e = &layout.edges[idx];
        if e.start_label.is_some() || e.end_label.is_some() {
            return Err(RoutingError::InvalidInput(
                "endpoint labels are not supported by alternative routing yet".into(),
            ));
        }
        let Some(l) = &e.label else {
            continue;
        };
        let mut choices = Vec::new();
        for s in e.points.windows(2) {
            let horizontal = (s[0].1 - s[1].1).abs() < 0.01;
            for f in [0.5, 0.25, 0.75, 0.1, 0.9] {
                let p = (
                    s[0].0 + (s[1].0 - s[0].0) * f,
                    s[0].1 + (s[1].1 - s[0].1) * f,
                );
                for shift in [0., -0.5, 0.5, -1., 1.] {
                    let p = if horizontal {
                        (p.0 + shift * l.width, p.1)
                    } else {
                        (p.0, p.1 + shift * l.height)
                    };
                    for gap in [10., 18., 26., 34., 42.] {
                        for sign in [-1., 1.] {
                            let a = if horizontal {
                                (p.0, p.1 + sign * (l.height / 2. + gap))
                            } else {
                                (p.0 + sign * (l.width / 2. + gap), p.1)
                            };
                            if !near(&e.points, a, l.width, l.height) {
                                continue;
                            }
                            let r = rect(a, l.width, l.height);
                            if occupied.iter().any(|&o| overlap(r, o)) || hit(&e.points, r) {
                                continue;
                            }
                            let crossings = layout
                                .edges
                                .iter()
                                .filter(|edge| hit(&edge.points, r))
                                .count();
                            let center_distance = e
                                .points
                                .windows(2)
                                .map(|s| {
                                    let x = a.0.clamp(s[0].0.min(s[1].0), s[0].0.max(s[1].0));
                                    let y = a.1.clamp(s[0].1.min(s[1].1), s[0].1.max(s[1].1));
                                    (x - a.0).abs() + (y - a.1).abs()
                                })
                                .fold(f32::INFINITY, f32::min);
                            choices.push((
                                crossings as f32 * 100000.
                                    + gap * 100.
                                    + center_distance
                                    + shift.abs() * 10.,
                                a,
                                r,
                            ));
                        }
                    }
                }
            }
        }
        choices.sort_by(|a, b| a.0.total_cmp(&b.0));
        let Some((_, anchor, r)) = choices.first() else {
            return Err(RoutingError::NoSpace(format!(
                "no attached label position for edge {idx}"
            )));
        };
        occupied.push(*r);
        layout.edges[idx].label_anchor = Some(*anchor);
    }
    Ok(())
}
pub(super) fn obstacles(layout: &Layout, first: u32) -> Vec<Obstacle> {
    layout
        .edges
        .iter()
        .filter_map(|e| Some((e.label.as_ref()?, e.label_anchor?)))
        .enumerate()
        .map(|(i, (l, a))| {
            let r = rect(a, l.width, l.height);
            Obstacle {
                id: first + i as u32,
                polygon: rectangle(r.0, r.1, r.2, r.3),
                ports: vec![],
            }
        })
        .collect()
}
pub(super) fn validate(layout: &Layout) -> Result<(), RoutingError> {
    let mut occupied = node_rects(layout);
    for (idx, e) in layout.edges.iter().enumerate() {
        let Some(l) = &e.label else {
            continue;
        };
        let a = e
            .label_anchor
            .ok_or_else(|| RoutingError::NoSpace(format!("missing label {idx}")))?;
        let r = rect(a, l.width, l.height);
        if occupied.iter().any(|&o| overlap(r, o))
            || layout.edges.iter().any(|edge| hit(&edge.points, r))
        {
            return Err(RoutingError::NoSpace(format!(
                "label collision on edge {idx}"
            )));
        }
        // The label must remain near its own route after routing around text.
        let near = near(&e.points, a, l.width, l.height);
        if !near {
            return Err(RoutingError::NoSpace(format!(
                "detached label on edge {idx}"
            )));
        }
        occupied.push(r);
    }
    Ok(())
}

pub(super) fn collisions(layout: &Layout) -> usize {
    let mut occupied = node_rects(layout);
    let mut count = 0;
    for e in &layout.edges {
        if let (Some(l), Some(a)) = (&e.label, e.label_anchor) {
            let r = rect(a, l.width, l.height);
            count += occupied.iter().filter(|&&o| overlap(r, o)).count();
            count += layout.edges.iter().filter(|e| hit(&e.points, r)).count();
            occupied.push(r);
        }
    }
    count
}
