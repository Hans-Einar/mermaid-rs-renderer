use super::*;
// The adapter currently accepts convex, non-degenerate obstacle polygons.
// This deliberately excludes malformed/self-intersecting geometry before FFI.
pub(super) fn obstacle(o: &Obstacle) -> bool {
    let mut sign = 0.;
    for i in 0..o.polygon.len() {
        let a = o.polygon[i];
        let b = o.polygon[(i + 1) % o.polygon.len()];
        let c = o.polygon[(i + 2) % o.polygon.len()];
        let cross = (b.0 - a.0) * (c.1 - b.1) - (b.1 - a.1) * (c.0 - b.0);
        if cross.abs() < 1e-8 {
            return false;
        }
        if sign == 0. {
            sign = cross;
        }
        if sign * cross < 0. {
            return false;
        }
        // Every vertex must lie on the same side of every edge, excluding stars.
        for p in &o.polygon {
            if ((b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0)) * sign < -1e-6 {
                return false;
            }
        }
    }
    let bounds = o.polygon.iter().fold(
        (
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ),
        |a, p| (a.0.min(p.0), a.1.min(p.1), a.2.max(p.0), a.3.max(p.1)),
    );
    o.ports.iter().all(|p| {
        p.point.0 >= bounds.0 - 0.001
            && p.point.0 <= bounds.2 + 0.001
            && p.point.1 >= bounds.1 - 0.001
            && p.point.1 <= bounds.3 + 0.001
    })
}
fn port_ok(obstacle: &Obstacle, mask: u32, point: Point, next: Point, slide: bool) -> bool {
    obstacle.ports.iter().any(|p| {
        let matches = ((p.point.0 - point.0).abs() < 0.01 && (p.point.1 - point.1).abs() < 0.01)
            || (slide && sliding_point(obstacle, p, point));
        let dx = next.0 - point.0;
        let dy = next.1 - point.1;
        let direction = if dx.abs() < 0.01 {
            if dy < 0. { UP } else { DOWN }
        } else if dx < 0. {
            LEFT
        } else {
            RIGHT
        };
        matches && p.directions & mask & direction != 0
    })
}
pub(super) fn routes(input: &RoutingInput, routes: &[Route]) -> Result<(), RoutingError> {
    for r in routes {
        if r.points
            .windows(2)
            .any(|s| (s[0].0 - s[1].0).abs() > 0.01 && (s[0].1 - s[1].1).abs() > 0.01)
        {
            return Err(RoutingError::Backend(format!(
                "non-orthogonal edge {} {:?}",
                r.id, r.points
            )));
        }
        let e = input.connections.iter().find(|e| e.id == r.id).unwrap();
        let source = input.obstacles.iter().find(|o| o.id == e.source).unwrap();
        let target = input.obstacles.iter().find(|o| o.id == e.target).unwrap();
        if !port_ok(
            source,
            e.source_directions,
            r.points[0],
            r.points[1],
            input.slide_ports,
        ) || !port_ok(
            target,
            e.target_directions,
            *r.points.last().unwrap(),
            r.points[r.points.len() - 2],
            input.slide_ports,
        ) {
            return Err(RoutingError::Backend(format!(
                "invalid selected port/direction: edge {} {:?}",
                r.id, r.points
            )));
        }
    }
    Ok(())
}

pub(super) fn sliding_rectangle(o: &Obstacle) -> bool {
    o.polygon.len() == 4
        && o.polygon
            .iter()
            .zip(o.polygon.iter().cycle().skip(1))
            .all(|(a, b)| a.0 == b.0 || a.1 == b.1)
        && o.ports.iter().all(|p| {
            [UP, DOWN, LEFT, RIGHT].contains(&p.directions) && sliding_point(o, p, p.point)
        })
}
fn sliding_point(o: &Obstacle, p: &Port, q: Point) -> bool {
    let (x0, y0, x1, y1) = o.polygon.iter().fold(
        (
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ),
        |a, p| (a.0.min(p.0), a.1.min(p.1), a.2.max(p.0), a.3.max(p.1)),
    );
    let in_x = q.0 >= x0 - 0.01 && q.0 <= x1 + 0.01;
    let in_y = q.1 >= y0 - 0.01 && q.1 <= y1 + 0.01;
    match p.directions {
        UP => (q.1 - y0).abs() < 0.01 && in_x,
        DOWN => (q.1 - y1).abs() < 0.01 && in_x,
        LEFT => (q.0 - x0).abs() < 0.01 && in_y,
        RIGHT => (q.0 - x1).abs() < 0.01 && in_y,
        _ => false,
    }
}
