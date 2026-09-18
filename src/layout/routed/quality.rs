//! Measurements on final routes, independent of the routing implementation.
use super::*;
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct Quality {
    pub length: f64,
    pub bends: usize,
    pub crossings: usize,
    pub parallel_overlaps: usize,
    pub close_parallel_segments: usize,
    pub node_traversals: usize,
    pub label_collisions: usize,
    pub attachment_intrusions: usize,
}
pub fn measure(layout: &Layout, separation: f64) -> Quality {
    let mut q = Quality::default();
    for e in &layout.edges {
        for s in e.points.windows(2) {
            q.length += (s[1].0 - s[0].0).abs() as f64 + (s[1].1 - s[0].1).abs() as f64;
            for n in layout.nodes.values().filter(|n| !n.hidden) {
                if super::geometry::interior(s[0], s[1], n) {
                    q.node_traversals += 1;
                }
            }
        }
        q.bends += e
            .points
            .windows(3)
            .filter(|s| {
                let a = (s[1].0 - s[0].0, s[1].1 - s[0].1);
                let b = (s[2].0 - s[1].0, s[2].1 - s[1].1);
                (a.0 * b.1 - a.1 * b.0).abs() > 0.01
            })
            .count();
    }
    for (i, a) in layout.edges.iter().enumerate() {
        for b in layout.edges.iter().skip(i + 1) {
            for s in a.points.windows(2) {
                for t in b.points.windows(2) {
                    let sh = (s[0].1 - s[1].1).abs() < 0.01;
                    let th = (t[0].1 - t[1].1).abs() < 0.01;
                    if sh == th {
                        let (sa, sb, ta, tb, d) = if sh {
                            (s[0].0, s[1].0, t[0].0, t[1].0, (s[0].1 - t[0].1).abs())
                        } else {
                            (s[0].1, s[1].1, t[0].1, t[1].1, (s[0].0 - t[0].0).abs())
                        };
                        if sa.max(sb).min(ta.max(tb)) - sa.min(sb).max(ta.min(tb)) > 0.01 {
                            if d < 0.01 {
                                q.parallel_overlaps += 1;
                            } else if (d as f64) < separation - 0.01 {
                                q.close_parallel_segments += 1;
                            }
                        }
                    } else {
                        let (h, v) = if sh { (s, t) } else { (t, s) };
                        if v[0].0 > h[0].0.min(h[1].0) + 0.01
                            && v[0].0 < h[0].0.max(h[1].0) - 0.01
                            && h[0].1 > v[0].1.min(v[1].1) + 0.01
                            && h[0].1 < v[0].1.max(v[1].1) - 0.01
                        {
                            q.crossings += 1;
                        }
                    }
                }
            }
        }
    }
    q.attachment_intrusions = super::attachment::intrusions(layout);
    q.label_collisions = super::labels::collisions(layout);
    q
}
