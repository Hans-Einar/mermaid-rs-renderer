//! Bounded presentation-only label leaders. No mutation of Layout or routes.
use crate::{LayoutConfig, Theme, layout::Layout};
type P = (f32, f32);
type Rect = (f32, f32, f32, f32);
#[derive(Clone)]
struct Candidate {
    side: usize,
    segment: usize,
    points: Vec<P>,
    length: f32,
}
fn distance(p: P, a: P, b: P) -> f32 {
    let d = (b.0 - a.0, b.1 - a.1);
    let t = (((p.0 - a.0) * d.0 + (p.1 - a.1) * d.1) / (d.0 * d.0 + d.1 * d.1).max(1e-12))
        .clamp(0., 1.);
    (p.0 - a.0 - t * d.0).hypot(p.1 - a.1 - t * d.1)
}
fn cross(a: P, b: P, c: P) -> f32 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}
fn separation(a: P, b: P, c: P, d: P) -> f32 {
    if cross(a, b, c) * cross(a, b, d) < 0. && cross(c, d, a) * cross(c, d, b) < 0. {
        return 0.;
    }
    distance(a, c, d)
        .min(distance(b, c, d))
        .min(distance(c, a, b))
        .min(distance(d, a, b))
}
fn hits(points: &[P], r: Rect) -> bool {
    let inside = |p: P| p.0 > r.0 && p.0 < r.0 + r.2 && p.1 > r.1 && p.1 < r.1 + r.3;
    let corners = [
        (r.0, r.1),
        (r.0 + r.2, r.1),
        (r.0 + r.2, r.1 + r.3),
        (r.0, r.1 + r.3),
        (r.0, r.1),
    ];
    points.windows(2).any(|s| {
        inside(s[0])
            || inside(s[1])
            || corners
                .windows(2)
                .any(|b| separation(s[0], s[1], b[0], b[1]) < 0.01)
    })
}
fn label_rect(e: &crate::layout::EdgeLayout, pad: P) -> Option<Rect> {
    let l = e.label.as_ref()?;
    let p = e.label_anchor?;
    Some((
        p.0 - l.width / 2. - pad.0,
        p.1 - l.height / 2. - pad.1,
        l.width + 2. * pad.0,
        l.height + 2. * pad.1,
    ))
}
fn candidates(points: &[P], r: Rect) -> Vec<Candidate> {
    let mut result = vec![];
    for (segment, s) in points.windows(2).enumerate() {
        for side in 0..2 {
            let sign = if side == 0 { -1. } else { 1. };
            let x = if side == 0 { r.0 } else { r.0 + r.2 };
            let p = if (s[0].0 - s[1].0).abs() < 0.01 {
                let y = r.1 + r.3 / 2.;
                if (s[0].0 - x) * sign <= 0.
                    || y < s[0].1.min(s[1].1) + 8.
                    || y > s[0].1.max(s[1].1) - 8.
                {
                    continue;
                }
                vec![(x, y), (s[0].0, y)]
            } else if (s[0].1 - s[1].1).abs() < 0.01 {
                let target = s[0].1;
                let y = if target < r.1 {
                    r.1
                } else if target > r.1 + r.3 {
                    r.1 + r.3
                } else {
                    continue;
                };
                let gap = (target - y).abs();
                let leg = 8_f32.min(gap / 2.);
                if leg < 2. {
                    continue;
                }
                let end_x = x + sign * leg;
                if end_x < s[0].0.min(s[1].0) + 8. || end_x > s[0].0.max(s[1].0) - 8. {
                    continue;
                }
                vec![
                    (x, y),
                    (end_x, y + (target - y).signum() * leg),
                    (end_x, target),
                ]
            } else {
                continue;
            };
            let length = p
                .windows(2)
                .map(|s| (s[0].0 - s[1].0).hypot(s[0].1 - s[1].1))
                .sum();
            if length <= 100. {
                result.push(Candidate {
                    side,
                    segment,
                    points: p,
                    length,
                });
            }
        }
    }
    result.sort_by(|a, b| {
        a.side
            .cmp(&b.side)
            .then(a.length.total_cmp(&b.length))
            .then(a.segment.cmp(&b.segment))
    });
    result
}

/// Add collision-checked flowchart leaders to library SVG. Returns the SVG and
/// the number of omitted leaders (blocked, missing anchor, or workload limit).
pub fn add_label_leaders(
    mut svg: String,
    layout: &Layout,
    theme: &Theme,
    config: &LayoutConfig,
) -> (String, usize) {
    if layout.kind != crate::ir::DiagramKind::Flowchart {
        return (svg, 0);
    }
    let count = layout.edges.iter().filter(|e| e.label.is_some()).count();
    if layout.edges.len() > 256 || layout.edges.iter().map(|e| e.points.len()).sum::<usize>() > 4096
    {
        return (svg, count);
    }
    let Some(end) = svg.rfind("</svg>") else {
        return (svg, count);
    };
    let pad = super::edge_label_padding(layout.kind, config);
    let boxes: Vec<_> = layout
        .nodes
        .values()
        .filter(|n| !n.hidden)
        .map(|n| (n.x - 2., n.y - 2., n.width + 4., n.height + 4.))
        .chain(
            layout
                .subgraphs
                .iter()
                .filter(|g| !g.label.is_empty())
                .map(|g| {
                    (
                        g.x + g.width / 2. - g.label_block.width / 2. - 2.,
                        g.y - 2.,
                        g.label_block.width + 4.,
                        g.label_block.height + 12.,
                    )
                }),
        )
        .collect();
    let mut chosen: Vec<Vec<P>> = vec![];
    let mut markup = String::new();
    let mut omitted = 0;
    for (idx, e) in layout.edges.iter().enumerate() {
        if e.label.is_none() {
            continue;
        }
        let Some(r) = label_rect(e, pad) else {
            omitted += 1;
            continue;
        };
        let choice = candidates(&e.points, r).into_iter().find(|c| {
            if boxes.iter().any(|&b| hits(&c.points, b)) {
                return false;
            }
            let tip = *c.points.last().unwrap();
            for (other, edge) in layout.edges.iter().enumerate() {
                if other != idx
                    && let Some(b) = label_rect(edge, (pad.0 + 2., pad.1 + 2.))
                    && hits(&c.points, b)
                {
                    return false;
                }
                for (segment, s) in edge.points.windows(2).enumerate() {
                    if other == idx && segment == c.segment {
                        continue;
                    }
                    if distance(tip, s[0], s[1]) < 8.
                        || c.points
                            .windows(2)
                            .any(|p| separation(p[0], p[1], s[0], s[1]) < 1.5)
                    {
                        return false;
                    }
                }
            }
            !chosen.iter().any(|other| {
                other.windows(2).any(|s| {
                    c.points
                        .windows(2)
                        .any(|p| separation(p[0], p[1], s[0], s[1]) < 3.)
                })
            })
        });
        let Some(c) = choice else {
            omitted += 1;
            continue;
        };
        let tip = *c.points.last().unwrap();
        let mut path = format!("M {},{}", c.points[0].0, c.points[0].1);
        for p in &c.points[1..] {
            path.push_str(&format!(" L {},{}", p.0, p.1));
        }
        let color = super::escape_xml(&theme.line_color);
        markup.push_str(&format!("<g class=\"label-leader\" data-edge-id=\"edge-{idx}\" pointer-events=\"none\"><path d=\"{path}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1\" stroke-linejoin=\"miter\" stroke-linecap=\"butt\"/><circle cx=\"{}\" cy=\"{}\" r=\"2.2\" fill=\"{color}\"/></g>",tip.0,tip.1));
        chosen.push(c.points);
    }
    svg.insert_str(end, &markup);
    (svg, omitted)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn left_preference_and_sharp_diagonal_geometry() {
        let c = candidates(
            &[(-20., -50.), (-20., 80.), (100., 80.)],
            (0., 0., 40., 20.),
        );
        assert_eq!(c[0].points, vec![(0., 10.), (-20., 10.)]);
        let c = candidates(&[(-50., -30.), (100., -30.)], (0., 0., 40., 20.));
        assert_eq!(c[0].points, vec![(0., 0.), (-8., -8.), (-8., -30.)]);
        assert_eq!(c[1].side, 1);
    }
    #[test]
    fn blocks_diagonal_crossings_and_near_touches() {
        assert_eq!(separation((0., 0.), (10., 10.), (0., 10.), (10., 0.)), 0.);
        assert!(hits(&[(0., 0.), (20., 20.)], (8., 8., 4., 4.)));
        assert!(!hits(&[(0., 0.), (20., 0.)], (8., 8., 4., 4.)));
    }
    #[test]
    fn svg_is_optional_and_preserves_routes() {
        let graph = crate::parse_mermaid_strict("flowchart TD\nA-->|label|B")
            .unwrap()
            .graph;
        let config = LayoutConfig::default();
        let theme = Theme::modern();
        let mut layout = crate::layout::compute_layout(&graph, &theme, &config);
        let mut obstacle = layout.nodes.values().next().unwrap().clone();
        layout.nodes.clear();
        layout.edges[0].points = vec![(-20., -50.), (-20., 80.)];
        layout.edges[0].label_anchor = Some((40., 10.));
        let before = layout.edges[0].points.clone();
        let (svg, omitted) = add_label_leaders("<svg></svg>".into(), &layout, &theme, &config);
        assert_eq!(omitted, 0);
        assert!(svg.contains("<circle"));
        assert!(!svg.contains(" Q "));
        assert_eq!(before, layout.edges[0].points);
        // Both sides can reach this U-shaped route. Block only the left leader.
        layout.edges[0].points = vec![(-20., -200.), (-20., 200.), (100., 200.), (100., -200.)];
        obstacle.x = -5.;
        obstacle.y = 5.;
        obstacle.width = 5.;
        obstacle.height = 10.;
        layout.nodes.insert("blocker".into(), obstacle.clone());
        let (svg, omitted) = add_label_leaders("<svg></svg>".into(), &layout, &theme, &config);
        assert_eq!(omitted, 0);
        assert!(svg.contains("cx=\"100\""));
        obstacle.x = 70.;
        obstacle.y = -100.;
        obstacle.width = 5.;
        obstacle.height = 200.;
        layout.nodes.insert("right-blocker".into(), obstacle);
        let (svg, omitted) = add_label_leaders("<svg></svg>".into(), &layout, &theme, &config);
        assert_eq!(omitted, 1);
        assert!(!svg.contains("label-leader"));
    }
}
