//! The open gap between a caption and its nearest overlapping route segment.
//! Coordinates on the owner segment are excluded from the protected rectangle.
use super::labels::Rect;

pub(super) fn zone(points: &[(f32, f32)], r: Rect) -> Option<Rect> {
    let mut candidates = Vec::new();
    for s in points.windows(2) {
        let (x0, x1) = (s[0].0.min(s[1].0) as f64, s[0].0.max(s[1].0) as f64);
        let (y0, y1) = (s[0].1.min(s[1].1) as f64, s[0].1.max(s[1].1) as f64);
        if (y1 - y0).abs() < 0.01 {
            let lo = x0.max(r.0);
            let hi = x1.min(r.0 + r.2);
            if hi <= lo {
                continue;
            }
            if y0 < r.1 {
                candidates.push((r.1 - y0, (lo, y0, hi - lo, r.1 - y0)));
            } else if y0 > r.1 + r.3 {
                candidates.push((y0 - r.1 - r.3, (lo, r.1 + r.3, hi - lo, y0 - r.1 - r.3)));
            }
        } else if (x1 - x0).abs() < 0.01 {
            let lo = y0.max(r.1);
            let hi = y1.min(r.1 + r.3);
            if hi <= lo {
                continue;
            }
            if x0 < r.0 {
                candidates.push((r.0 - x0, (x0, lo, r.0 - x0, hi - lo)));
            } else if x0 > r.0 + r.2 {
                candidates.push((x0 - r.0 - r.2, (r.0 + r.2, lo, x0 - r.0 - r.2, hi - lo)));
            }
        }
    }
    candidates
        .into_iter()
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|v| v.1)
}

/// Extend the caption obstacle towards its owner, leaving room for libavoid's
/// global shape buffer. Validation still checks the entire unbuffered gap.
pub(super) fn obstacle(r: Rect, z: Rect, clearance: f64) -> Rect {
    let keep = clearance;
    let (mut x0, mut y0, mut x1, mut y1) = (r.0, r.1, r.0 + r.2, r.1 + r.3);
    if z.0 < r.0 {
        x0 = (z.0 + keep).min(x0);
    }
    if z.1 < r.1 {
        y0 = (z.1 + keep).min(y0);
    }
    if z.0 + z.2 > x1 {
        x1 = (z.0 + z.2 - keep).max(x1);
    }
    if z.1 + z.3 > y1 {
        y1 = (z.1 + z.3 - keep).max(y1);
    }
    (x0, y0, x1 - x0, y1 - y0)
}

pub(super) fn intrusions(layout: &super::Layout) -> usize {
    layout
        .edges
        .iter()
        .enumerate()
        .map(|(idx, e)| {
            let Some((label, a)) = e.label.as_ref().zip(e.label_anchor) else {
                return 0;
            };
            let r = super::labels::rect(a, label.width, label.height);
            let Some(z) = zone(&e.points, r) else {
                return 1;
            };
            layout
                .edges
                .iter()
                .enumerate()
                .filter(|(i, o)| *i != idx && super::labels::hit(&o.points, z))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::super::labels::hit;
    use super::*;
    #[test]
    fn blocks_parallel_and_crossing_intruders_but_not_owner() {
        let owner = [(40., 0.), (40., 100.)];
        let z = zone(&owner, (0., 20., 20., 40.)).unwrap();
        assert_eq!(z, (20., 20., 20., 40.));
        assert!(!hit(&owner, z));
        assert!(hit(&[(30., 0.), (30., 100.)], z));
        assert!(hit(&[(10., 40.), (50., 40.)], z));
        assert!(!hit(&[(10., 80.), (50., 80.)], z));
        let o = obstacle((0., 20., 20., 40.), z, 4.);
        assert!((o.0 + o.2 - 36.).abs() < 1e-6);
    }
    #[test]
    fn rejects_intruder_even_inside_old_distance_tolerance() {
        use crate::layout::{TextBlock, compute_layout};
        use crate::{LayoutConfig, Theme, parse_mermaid_strict};
        let graph = parse_mermaid_strict("flowchart TD\nA-->|owner|B\nA-->C")
            .unwrap()
            .graph;
        let mut layout = compute_layout(&graph, &Theme::modern(), &LayoutConfig::default());
        layout.nodes.clear();
        layout.subgraphs.clear();
        layout.edges[0].points = vec![(40., 0.), (40., 100.)];
        layout.edges[0].label = Some(TextBlock {
            lines: vec!["owner".into()],
            width: 20.,
            height: 20.,
        });
        layout.edges[0].label_anchor = Some((10., 40.));
        layout.edges[1].points = vec![(39., 0.), (39., 100.)];
        layout.edges[1].label = None;
        let error = super::super::labels::validate(&layout).unwrap_err();
        assert!(error.to_string().contains("attachment intrusion"));
        layout.edges[1].points = vec![(50., 0.), (50., 100.)];
        super::super::labels::validate(&layout).unwrap();
    }
    #[test]
    fn requires_projection_and_supports_both_axes_and_sides() {
        assert!(zone(&[(0., 0.), (10., 0.)], (20., 20., 10., 10.)).is_none());
        for (p, r, expected) in [
            (
                vec![(0., 0.), (100., 0.)],
                (20., 20., 40., 10.),
                (20., 0., 40., 20.),
            ),
            (
                vec![(100., 50.), (0., 50.)],
                (20., 20., 40., 10.),
                (20., 30., 40., 20.),
            ),
            (
                vec![(0., 100.), (0., 0.)],
                (20., 20., 40., 10.),
                (0., 20., 20., 10.),
            ),
        ] {
            assert_eq!(zone(&p, r), Some(expected));
        }
    }
}
