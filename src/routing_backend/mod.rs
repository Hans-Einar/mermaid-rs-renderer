//! Backend-independent, owned routing values. IDs are not source/target pairs.
use std::time::{Duration, Instant};
#[cfg(feature = "libavoid")]
mod libavoid;
#[cfg(feature = "libavoid")]
pub use libavoid::Libavoid;
pub type Point = (f64, f64);
#[derive(Clone, Debug)]
pub struct Port { pub point: Point, pub directions: u32 }
#[derive(Clone, Debug)]
pub struct Obstacle { pub id: u32, pub polygon: Vec<Point>, pub ports: Vec<Port> }
#[derive(Clone, Debug)]
pub struct Connection { pub id: u32, pub source: u32, pub target: u32, pub source_directions: u32, pub target_directions: u32 }
#[derive(Clone, Debug)]
pub struct RoutingInput {
    pub obstacles: Vec<Obstacle>, pub connections: Vec<Connection>,
    pub clearance: f64, pub separation: f64, pub bend_cost: f64,
}
#[derive(Clone, Debug)]
pub struct Route { pub id: u32, pub points: Vec<Point>, pub source_port: Point, pub target_port: Point }
#[derive(Clone, Debug)]
pub struct RoutingOutput { pub routes: Vec<Route>, pub elapsed: Duration, pub diagnostics: Vec<String> }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutingError { InvalidInput(String), Unavailable, Cancelled, BudgetExceeded, Backend(String), NoSpace(String) }
impl std::fmt::Display for RoutingError { fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {write!(f,"{self:?}")} }
impl std::error::Error for RoutingError {}
pub struct RoutingControl<'a> { pub deadline: Instant, pub cancelled: &'a dyn Fn() -> bool }
impl RoutingControl<'_> {
    pub fn check(&self)->Result<(),RoutingError> {
        if (self.cancelled)() {Err(RoutingError::Cancelled)}
        else if Instant::now() >= self.deadline {Err(RoutingError::BudgetExceeded)} else {Ok(())}
    }
}
pub trait RoutingBackend {
    fn route(&self, input: &RoutingInput, control: &RoutingControl<'_>) -> Result<RoutingOutput, RoutingError>;
}
