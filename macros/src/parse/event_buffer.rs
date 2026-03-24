//! Provider of [`EventBuffer`].

use crate::*;
use pulldown_cmark::Event;
use std::cmp::Ordering;
use std::mem;

/// Event buffer.
///
/// This type buffers input events.
/// And then split them to block level.
#[derive(Default)]
pub(crate) struct EventBuffer<'a> {
    /// Buffer.
    buf: Vec<Event<'a>>,
    /// Nest level.
    nest_lv: u32,
}

impl<'a> EventBuffer<'a> {
    /// Creates a new instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Input event and returns block if it is created.
    ///
    /// # Panics
    ///
    /// Panics if event nest level is overflowed or underflowed.
    pub fn input(&mut self, event: Event<'a>) -> Option<Vec<Event<'a>>> {
        debug_assert!((self.nest_lv == 0) == self.buf.is_empty());

        // Get increment value of nest level.
        let nest_up = match &event {
            Event::Start(_) => 1,
            Event::End(_) => -1,
            _ => 0,
        };

        // Increment the nest level, and report if overflow occurs.
        let Some(nest_lv) = self.nest_lv.checked_add_signed(nest_up) else {
            panic!(
                "{}",
                match nest_up.cmp(&0) {
                    Ordering::Equal => unreachable!(),
                    Ordering::Less => msg::EVENT_NEST_LV_UNDERFLOW,
                    Ordering::Greater => msg::EVENT_NEST_LV_OVERFLOW,
                }
            )
        };

        // Append event information.
        self.buf.push(event);
        self.nest_lv = nest_lv;

        // Returns block.
        if self.nest_lv == 0 {
            let old_buf = mem::take(&mut self.buf);
            return Some(old_buf);
        }

        None
    }
}
