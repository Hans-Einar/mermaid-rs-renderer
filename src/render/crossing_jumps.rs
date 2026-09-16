//! Optional presentation only: proper crossings become small quadratic bridges.
//! Edge indices provide deterministic priority. Logical points never change.
use crate::layout::Layout;
use std::collections::BTreeMap;
type Point = (f32, f32);
#[derive(Clone, Copy, Debug)]
pub struct CrossingJumps {
    pub radius: f32,
}
impl Default for CrossingJumps {
    fn default() -> Self {
        Self { radius: 4. }
    }
}
#[derive(Clone, Copy, Debug)]
struct Jump {
    segment: usize,
    point: Point,
}
fn proper(a: &[Point], b: &[Point], margin: f32) -> Option<Point> {
    if [a, b]
        .iter()
        .any(|s| (s[0].0 - s[1].0).abs() > 0.01 && (s[0].1 - s[1].1).abs() > 0.01)
    {
        return None;
    }
    let ah = (a[0].1 - a[1].1).abs() < 0.01;
    let bh = (b[0].1 - b[1].1).abs() < 0.01;
    if ah == bh {
        return None;
    }
    let (h, v) = if ah { (a, b) } else { (b, a) };
    let p = (v[0].0, h[0].1);
    (p.0 > h[0].0.min(h[1].0) + margin
        && p.0 < h[0].0.max(h[1].0) - margin
        && p.1 > v[0].1.min(v[1].1) + margin
        && p.1 < v[0].1.max(v[1].1) - margin)
        .then_some(p)
}
fn distance(p: Point, a: Point, b: Point) -> f32 {
    (p.0 - p.0.clamp(a.0.min(b.0), a.0.max(b.0))).abs()
        + (p.1 - p.1.clamp(a.1.min(b.1), a.1.max(b.1))).abs()
}
fn inside(p: Point, x: f32, y: f32, w: f32, h: f32, margin: f32) -> bool {
    p.0 > x - margin && p.0 < x + w + margin && p.1 > y - margin && p.1 < y + h + margin
}
pub(super) fn paths(layout: &Layout, options: CrossingJumps) -> BTreeMap<usize, String> {
    if layout.edges.iter().map(|e| e.points.len()).sum::<usize>() > 4096 {
        return BTreeMap::new();
    }
    let radius = if options.radius.is_finite() {
        options.radius.clamp(1., 8.)
    } else {
        4.
    };
    let margin = radius + 12.;
    let mut jumps: BTreeMap<usize, Vec<Jump>> = BTreeMap::new();
    let mut reserved: Vec<Point> = Vec::new();
    for (i, a) in layout.edges.iter().enumerate() {
        for (j, b) in layout.edges.iter().enumerate().skip(i + 1) {
            for (si, s) in a.points.windows(2).enumerate() {
                for (ti, t) in b.points.windows(2).enumerate() {
                    let Some(p) = proper(s, t, margin) else {
                        continue;
                    };
                    if reserved
                        .iter()
                        .any(|q| (q.0 - p.0).abs() + (q.1 - p.1).abs() < margin * 2.)
                    {
                        continue;
                    }
                    if layout
                        .nodes
                        .values()
                        .filter(|n| !n.hidden)
                        .any(|n| inside(p, n.x, n.y, n.width, n.height, margin))
                    {
                        continue;
                    }
                    if layout
                        .edges
                        .iter()
                        .any(|e| match (&e.label, e.label_anchor) {
                            (Some(l), Some(a)) => inside(
                                p,
                                a.0 - l.width / 2.,
                                a.1 - l.height / 2.,
                                l.width,
                                l.height,
                                margin,
                            ),
                            _ => false,
                        })
                    {
                        continue;
                    }
                    if layout.subgraphs.iter().any(|g| {
                        inside(
                            p,
                            g.x + g.width / 2. - g.label_block.width / 2.,
                            g.y,
                            g.label_block.width,
                            g.label_block.height + 8.,
                            margin,
                        )
                    }) {
                        continue;
                    }
                    // Reserve the whole bridge box against third segments, including
                    // junctions and another part of either crossing route.
                    if layout.edges.iter().enumerate().any(|(k, e)| {
                        e.points.windows(2).enumerate().any(|(u, v)| {
                            !((k == i && u == si) || (k == j && u == ti))
                                && distance(p, v[0], v[1]) < margin
                        })
                    }) {
                        continue;
                    }
                    // Higher edge ID goes over; shared endpoints/collinear segments
                    // have already been excluded by the strict interior intersection.
                    jumps.entry(j).or_default().push(Jump {
                        segment: ti,
                        point: p,
                    });
                    reserved.push(p);
                }
            }
        }
    }
    jumps
        .into_iter()
        .map(|(id, js)| (id, path(&layout.edges[id].points, &js, radius)))
        .collect()
}
fn toward(a: Point, b: Point, d: f32) -> Point {
    let len = (b.0 - a.0).hypot(b.1 - a.1);
    if len == 0. {
        a
    } else {
        (a.0 + (b.0 - a.0) * d / len, a.1 + (b.1 - a.1) * d / len)
    }
}
fn path(points: &[Point], jumps: &[Jump], radius: f32) -> String {
    use std::fmt::Write;
    let mut d = format!("M {} {}", points[0].0, points[0].1);
    for (idx, s) in points.windows(2).enumerate() {
        let mut js: Vec<_> = jumps.iter().filter(|j| j.segment == idx).collect();
        js.sort_by(|a, b| {
            ((a.point.0 - s[0].0).abs() + (a.point.1 - s[0].1).abs())
                .total_cmp(&((b.point.0 - s[0].0).abs() + (b.point.1 - s[0].1).abs()))
        });
        for j in js {
            let before = toward(j.point, s[0], radius);
            let after = toward(j.point, s[1], radius);
            let c = if (s[0].1 - s[1].1).abs() < 0.01 {
                (j.point.0, j.point.1 - radius * 2.)
            } else {
                (j.point.0 + radius * 2., j.point.1)
            };
            write!(
                d,
                " L {} {} Q {} {} {} {}",
                before.0, before.1, c.0, c.1, after.0, after.1
            )
            .unwrap();
        }
        if idx + 2 < points.len() {
            let next = points[idx + 2];
            let r = 10_f32
                .min((s[1].0 - s[0].0).hypot(s[1].1 - s[0].1) / 2.)
                .min((next.0 - s[1].0).hypot(next.1 - s[1].1) / 2.);
            let before = toward(s[1], s[0], r);
            let after = toward(s[1], next, r);
            write!(
                d,
                " L {} {} Q {} {} {} {}",
                before.0, before.1, s[1].0, s[1].1, after.0, after.1
            )
            .unwrap();
        } else {
            write!(d, " L {} {}", s[1].0, s[1].1).unwrap();
        }
    }
    d
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_proper_interior_crossings() {
        let h = [(0., 50.), (100., 50.)];
        assert_eq!(proper(&h, &[(50., 0.), (50., 100.)], 16.), Some((50., 50.)));
        assert_eq!(proper(&h, &[(0., 0.), (0., 100.)], 16.), None);
        assert_eq!(proper(&h, &[(10., 50.), (90., 50.)], 16.), None);
        assert_eq!(proper(&h, &[(90., 0.), (90., 100.)], 16.), None);
    }
    #[test]
    fn path_bridges_without_changing_endpoints() {
        let points = [(0., 50.), (100., 50.)];
        let d = path(
            &points,
            &[Jump {
                segment: 0,
                point: (50., 50.),
            }],
            4.,
        );
        assert_eq!(d, "M 0 50 L 46 50 Q 50 42 54 50 L 100 50");
    }
    #[test]
    fn deterministic_priority_and_third_segment_clearance() {
        let graph = crate::parse_mermaid_strict("flowchart TD\n A-->B\n C-->D")
            .unwrap()
            .graph;
        let mut l = crate::compute_layout(
            &graph,
            &crate::Theme::modern(),
            &crate::LayoutConfig::default(),
        );
        l.nodes.clear();
        l.edges[0].points = vec![(0., 50.), (100., 50.)];
        l.edges[1].points = vec![(50., 0.), (50., 100.)];
        let before = l.edges[1].points.clone();
        let p = paths(&l, CrossingJumps::default());
        assert_eq!(p.len(), 1);
        assert!(p.contains_key(&1));
        assert_eq!(l.edges[1].points, before);
        assert_eq!(p, paths(&l, CrossingJumps::default()));
        let mut extra = l.edges[0].clone();
        extra.points = vec![(55., 0.), (55., 100.)];
        l.edges.push(extra);
        assert!(paths(&l, CrossingJumps::default()).is_empty());
    }
}
