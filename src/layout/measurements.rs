//! Scoped premeasured labels and cooperative layout deadlines. No GUI types.
use super::{Layout, TextBlock, compute_layout};
use crate::{LayoutConfig, Theme, ir::Graph};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    time::{Duration, Instant},
};
thread_local! { static LABELS: RefCell<Option<HashMap<String, TextBlock>>> = const { RefCell::new(None) }; }
thread_local! { static DEADLINE: RefCell<Option<Instant>> = const { RefCell::new(None) }; }
thread_local! { static CHECKPOINTS: Cell<u32> = const { Cell::new(0) }; }
pub fn checkpoint() {
    // Geometry loops call this very frequently. Sample the clock every 64
    // checkpoints while retaining every loop boundary as a cancellation site.
    if CHECKPOINTS.with(|count| {
        let previous = count.get();
        count.set(previous.wrapping_add(1));
        previous & 63 != 0
    }) {
        return;
    }
    if DEADLINE.with(|d| {
        d.borrow()
            .is_some_and(|deadline| Instant::now() >= deadline)
    }) {
        std::panic::panic_any("Diagram layout time budget exceeded");
    }
}
pub fn lookup(text: &str) -> Option<TextBlock> {
    LABELS.with(|slot| {
        slot.borrow().as_ref().map(|labels| {
            labels
                .get(text)
                .unwrap_or_else(|| panic!("Missing measured label: {text}"))
                .clone()
        })
    })
}
pub fn layout(
    graph: &Graph,
    theme: &Theme,
    config: &LayoutConfig,
    labels: HashMap<String, TextBlock>,
    budget: Duration,
) -> Layout {
    with_measurements(labels, budget, || compute_layout(graph, theme, config))
}
/// Run either routing strategy with external text measurements. Thread-local
/// state is restored on success, error and unwinding, including nested calls.
pub fn with_measurements<T>(
    labels: HashMap<String, TextBlock>,
    budget: Duration,
    operation: impl FnOnce() -> T,
) -> T {
    struct Restore(Option<HashMap<String, TextBlock>>, Option<Instant>);
    impl Drop for Restore {
        fn drop(&mut self) {
            LABELS.with(|slot| *slot.borrow_mut() = self.0.take());
            DEADLINE.with(|slot| *slot.borrow_mut() = self.1.take());
        }
    }
    let _restore = Restore(
        LABELS.with(|slot| slot.replace(Some(labels))),
        DEADLINE.with(|slot| slot.replace(Some(Instant::now() + budget))),
    );
    CHECKPOINTS.with(|count| count.set(0));
    operation()
}
