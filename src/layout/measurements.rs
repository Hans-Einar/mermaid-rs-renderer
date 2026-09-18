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

/// A synchronous, borrowed measurer. No reference survives the scoped call.
/// Each callback returns layout pixels; failure is propagated through the
/// embedding application's unwind boundary, never through a C ABI callback.
pub type MeasureCallback = fn(usize, &str, f32, f32, bool) -> TextBlock;
thread_local! { static MEASURER: Cell<Option<(usize, MeasureCallback)>> = const { Cell::new(None) }; }
pub fn dynamic_lookup(text: &str, font_size: f32, max_width: f32, wrap: bool) -> Option<TextBlock> {
    checkpoint();
    MEASURER.with(|slot| {
        slot.get()
            .map(|(context, callback)| callback(context, text, font_size, max_width, wrap))
    })
}
pub fn with_measurer<T>(
    context: usize,
    callback: MeasureCallback,
    budget: Duration,
    operation: impl FnOnce() -> T,
) -> T {
    struct Restore(Option<(usize, MeasureCallback)>, Option<Instant>);
    impl Drop for Restore {
        fn drop(&mut self) {
            MEASURER.with(|slot| slot.set(self.0));
            DEADLINE.with(|slot| *slot.borrow_mut() = self.1.take());
        }
    }
    let _restore = Restore(
        MEASURER.with(|slot| slot.replace(Some((context, callback)))),
        DEADLINE.with(|slot| slot.replace(Some(Instant::now() + budget))),
    );
    CHECKPOINTS.with(|count| count.set(0));
    operation()
}

#[cfg(test)]
mod dynamic_tests {
    use super::*;
    fn measure(context: usize, text: &str, size: f32, _: f32, _: bool) -> TextBlock {
        TextBlock {
            lines: vec![text.into()],
            width: context as f32,
            height: size,
        }
    }
    #[test]
    fn borrowed_measurer_is_scoped_and_restored_after_unwind() {
        assert!(dynamic_lookup("x", 16., 200., true).is_none());
        with_measurer(40, measure, Duration::from_secs(1), || {
            assert_eq!(dynamic_lookup("x", 20., 200., true).unwrap().width, 40.);
            let _ = std::panic::catch_unwind(|| {
                with_measurer(70, measure, Duration::from_secs(1), || {
                    assert_eq!(dynamic_lookup("x", 20., 200., true).unwrap().height, 20.);
                    panic!("test");
                })
            });
            assert_eq!(dynamic_lookup("x", 20., 200., true).unwrap().width, 40.);
        });
        assert!(dynamic_lookup("x", 16., 200., true).is_none());
    }
}
