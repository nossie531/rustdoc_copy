//! Provider of [`MdSplitter`].

use crate::util::md_tool::md_print::*;
use pulldown_cmark::Event;
use std::iter::Peekable;

/// Markdown splitter.
///
/// This type was introduced to handle subtle problems in Markdown reprint.
/// Broken links are printed like `\[link\]`. Here, Rustdoc API links are
/// treated in the same way because they sometimes lack a URL. Therefore,
/// the broken link must be handled separately. This type splits the
/// sequence of Markdown events for it.
pub(crate) struct MdSplitter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    /// Markdown events.
    events: Peekable<I>,
}

impl<'a, I> MdSplitter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    /// Creates an new instance.
    pub fn new(events: I) -> Self {
        Self {
            events: events.peekable(),
        }
    }
}

impl<'a, I> Iterator for MdSplitter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = MdPartition<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        MdPartition::new(&mut self.events)
    }
}
