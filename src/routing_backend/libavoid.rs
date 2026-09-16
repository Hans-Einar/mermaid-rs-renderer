use super::*;
use std::{
    collections::BTreeSet,
    ffi::{c_char, c_void},
    panic::{AssertUnwindSafe, catch_unwind},
};
#[repr(C)]
#[derive(Clone, Copy)]
struct Pt {
    x: f64,
    y: f64,
}
#[repr(C)]
struct Pin {
    p: Pt,
    dirs: u32,
}
#[repr(C)]
struct Shape {
    id: u32,
    first: u32,
    count: u32,
    first_pin: u32,
    pin_count: u32,
}
#[repr(C)]
struct Edge {
    id: u32,
    source: u32,
    target: u32,
    source_dirs: u32,
    target_dirs: u32,
    source_pin: Pt,
    target_pin: Pt,
    locked: u32,
}
unsafe extern "C" {
    fn mermaid_avoid_route(
        points: *const Pt,
        pins: *const Pin,
        shapes: *const Shape,
        shape_count: usize,
        edges: *const Edge,
        edge_count: usize,
        clearance: f64,
        separation: f64,
        bend_cost: f64,
        slide_ports: u8,
        check: extern "C" fn(*mut c_void) -> u8,
        emit: extern "C" fn(*mut c_void, u32, *const Pt, usize) -> u8,
        context: *mut c_void,
        error: *mut c_char,
        error_size: usize,
    ) -> i32;
}
struct Context<'a, 'b> {
    control: &'a RoutingControl<'b>,
    routes: Vec<Route>,
    error: Option<RoutingError>,
}
extern "C" fn check(raw: *mut c_void) -> u8 {
    // The context lives for the entire synchronous C++ call and is never shared.
    let ctx = unsafe { &mut *raw.cast::<Context<'_, '_>>() };
    match catch_unwind(AssertUnwindSafe(|| ctx.control.check())) {
        Ok(Ok(())) if ctx.error.is_none() => 1,
        Ok(Err(e)) => {
            ctx.error = Some(e);
            0
        }
        Err(_) => {
            ctx.error = Some(RoutingError::Backend(
                "cancellation callback panicked".into(),
            ));
            0
        }
        _ => 0,
    }
}
extern "C" fn emit(raw: *mut c_void, id: u32, ptr: *const Pt, len: usize) -> u8 {
    let ctx = unsafe { &mut *raw.cast::<Context<'_, '_>>() };
    let result = catch_unwind(AssertUnwindSafe(|| {
        if ptr.is_null() || !(2..=65536).contains(&len) {
            return false;
        }
        let mut points: Vec<_> = unsafe { std::slice::from_raw_parts(ptr, len) }
            .iter()
            .map(|p| (p.x, p.y))
            .collect();
        points.dedup();
        if points.len() < 2 || points.iter().any(|p| !valid_point(*p)) {
            return false;
        }
        ctx.routes.push(Route {
            id,
            source_port: points[0],
            target_port: *points.last().unwrap(),
            points,
        });
        true
    }));
    if matches!(result, Ok(true)) {
        1
    } else {
        ctx.error = Some(RoutingError::Backend(
            "invalid route or output callback failure".into(),
        ));
        0
    }
}
fn valid_point(p: Point) -> bool {
    p.0.is_finite() && p.1.is_finite() && p.0.abs() < 1e7 && p.1.abs() < 1e7
}
pub struct Libavoid;
impl RoutingBackend for Libavoid {
    fn route(
        &self,
        input: &RoutingInput,
        control: &RoutingControl<'_>,
    ) -> Result<RoutingOutput, RoutingError> {
        control.check()?;
        let start = Instant::now();
        let invalid = |s: &str| RoutingError::InvalidInput(s.into());
        if input.obstacles.len() > 1024 || input.connections.len() > 4096 {
            return Err(invalid("diagram size limit"));
        }
        if [input.clearance, input.separation, input.bend_cost]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0 || *v > 1000.0)
        {
            return Err(invalid("invalid routing cost/clearance"));
        }
        let mut ids = BTreeSet::new();
        let mut points = Vec::new();
        let mut pins = Vec::new();
        let mut shapes = Vec::new();
        for o in &input.obstacles {
            if !ids.insert(o.id)
                || !(3..=128).contains(&o.polygon.len())
                || o.ports.len() > 128
                || o.polygon.iter().any(|p| !valid_point(*p))
                || o.ports
                    .iter()
                    .any(|p| !valid_point(p.point) || p.directions == 0 || p.directions > 15)
            {
                return Err(invalid("invalid obstacle/ports/identity"));
            }
            if !super::validation::obstacle(o) {
                return Err(invalid(
                    "non-convex/degenerate obstacle or out-of-bounds pin",
                ));
            }
            if input.slide_ports && !o.ports.is_empty() && !super::validation::sliding_rectangle(o)
            {
                return Err(invalid(
                    "sliding ports require axis-aligned rectangular boundaries",
                ));
            }
            shapes.push(Shape {
                id: o.id,
                first: points.len() as u32,
                count: o.polygon.len() as u32,
                first_pin: pins.len() as u32,
                pin_count: o.ports.len() as u32,
            });
            points.extend(o.polygon.iter().map(|p| Pt { x: p.0, y: p.1 }));
            pins.extend(o.ports.iter().map(|p| Pin {
                p: Pt {
                    x: p.point.0,
                    y: p.point.1,
                },
                dirs: p.directions,
            }));
        }
        let mut edge_ids = BTreeSet::new();
        let mut edges = Vec::new();
        for e in &input.connections {
            if !edge_ids.insert(e.id) {
                return Err(invalid("duplicate edge ID"));
            }
            for (id, mask) in [
                (e.source, e.source_directions),
                (e.target, e.target_directions),
            ] {
                if mask == 0
                    || mask > 15
                    || !input
                        .obstacles
                        .iter()
                        .any(|o| o.id == id && o.ports.iter().any(|p| p.directions & mask != 0))
                {
                    return Err(invalid("missing endpoint or allowed pin"));
                }
            }
            for (id, mask, p) in [
                (e.source, e.source_directions, e.source_port),
                (e.target, e.target_directions, e.target_port),
            ] {
                if let Some(p) = p {
                    if !valid_point(p)
                        || !input
                            .obstacles
                            .iter()
                            .find(|o| o.id == id)
                            .unwrap()
                            .ports
                            .iter()
                            .any(|pin| {
                                (pin.point.0 - p.0).abs() < 0.001
                                    && (pin.point.1 - p.1).abs() < 0.001
                                    && pin.directions & mask != 0
                            })
                    {
                        return Err(invalid("fixed port not offered by node"));
                    }
                }
            }
            edges.push(Edge {
                id: e.id,
                source: e.source,
                target: e.target,
                source_dirs: e.source_port.map_or(e.source_directions, |p| {
                    input
                        .obstacles
                        .iter()
                        .find(|o| o.id == e.source)
                        .unwrap()
                        .ports
                        .iter()
                        .find(|pin| {
                            (pin.point.0 - p.0).abs() < 0.001 && (pin.point.1 - p.1).abs() < 0.001
                        })
                        .unwrap()
                        .directions
                        & e.source_directions
                }),
                target_dirs: e.target_port.map_or(e.target_directions, |p| {
                    input
                        .obstacles
                        .iter()
                        .find(|o| o.id == e.target)
                        .unwrap()
                        .ports
                        .iter()
                        .find(|pin| {
                            (pin.point.0 - p.0).abs() < 0.001 && (pin.point.1 - p.1).abs() < 0.001
                        })
                        .unwrap()
                        .directions
                        & e.target_directions
                }),
                source_pin: Pt {
                    x: e.source_port.unwrap_or_default().0,
                    y: e.source_port.unwrap_or_default().1,
                },
                target_pin: Pt {
                    x: e.target_port.unwrap_or_default().0,
                    y: e.target_port.unwrap_or_default().1,
                },
                locked: u32::from(e.source_port.is_some())
                    | (u32::from(e.target_port.is_some()) << 1),
            });
        }
        let mut ctx = Context {
            control,
            routes: Vec::new(),
            error: None,
        };
        let mut error = [0 as c_char; 1024];
        let status = unsafe {
            mermaid_avoid_route(
                points.as_ptr(),
                pins.as_ptr(),
                shapes.as_ptr(),
                shapes.len(),
                edges.as_ptr(),
                edges.len(),
                input.clearance,
                input.separation,
                input.bend_cost,
                u8::from(input.slide_ports),
                check,
                emit,
                (&mut ctx as *mut Context<'_, '_>).cast(),
                error.as_mut_ptr(),
                error.len(),
            )
        };
        if let Some(e) = ctx.error {
            return Err(e);
        }
        control.check()?;
        if status != 0 {
            return Err(RoutingError::Backend(
                unsafe { std::ffi::CStr::from_ptr(error.as_ptr()) }
                    .to_string_lossy()
                    .into_owned(),
            ));
        }
        if ctx.routes.len() != edges.len()
            || ctx.routes.iter().map(|r| r.id).collect::<BTreeSet<_>>() != edge_ids
        {
            return Err(RoutingError::Backend("edge identity mismatch".into()));
        }
        super::validation::routes(input, &ctx.routes)?;
        Ok(RoutingOutput {
            routes: ctx.routes,
            elapsed: start.elapsed(),
            diagnostics: vec![],
        })
    }
}
