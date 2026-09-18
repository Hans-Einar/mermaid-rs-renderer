//! Bounded BoxUI 0.1 model and SVG preparation. No host or domain execution.
mod frame;
mod layout;
mod model;
mod parse;
mod svg;
pub use frame::{ChildScene, Control, Frame, Rect, Snapshot, ValueState};
pub use layout::prepare;
pub use model::{Binding, Document, Kind, Node, Size};
pub use parse::{ParseOutput, parse, validate};

#[cfg(test)]
mod tests;
