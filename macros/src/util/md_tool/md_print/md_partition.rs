use crate::{msg, util::md_tool::md_print::*};
use pulldown_cmark::{Event, LinkType, Tag, TagEnd};
use pulldown_cmark_to_cmark::State;
use std::iter::Peekable;

/// Markdown partition.
///
/// About backgrounds: See document of [`MdSplitter`].
pub(crate) struct MdPartition<'a> {
    /// Markdown type.
    md_type: MdType,
    /// Markdown events.
    events: Vec<Event<'a>>,
}

impl<'a> MdPartition<'a> {
    /// Creates a new instance.
    pub fn new<I>(events: &mut Peekable<I>) -> Option<Self>
    where
        I: Iterator<Item = Event<'a>>,
    {
        Some(match Self::is_broken_link_start(events.peek()?) {
            false => Self {
                md_type: MdType::Normal,
                events: Self::read_normal(events),
            },
            true => Self {
                md_type: MdType::BrokenLink,
                events: Self::read_broken_link(events),
            },
        })
    }

    /// Prints Markdown.
    pub fn print(&self, state: State<'a>) -> PrintOut<'a> {
        match self.md_type {
            MdType::Normal => self.print_normal(state),
            MdType::BrokenLink => self.print_broken_link(state),
        }
    }

    /// Returns `true` if given event is not included in normal part.
    fn is_not_normal(event: &Event) -> bool {
        Self::is_broken_link_start(event)
    }

    /// Returns `true` if given event is broken link start event.
    fn is_broken_link_start(event: &Event) -> bool {
        let Event::Start(tag) = event else {
            return false;
        };

        let Tag::Link { link_type, .. } = tag else {
            return false;
        };

        #[rustfmt::skip]
        return matches!(
            link_type,
            | LinkType::ReferenceUnknown
            | LinkType::CollapsedUnknown
            | LinkType::ShortcutUnknown
        );
    }

    /// Returns `true` if given event is broken link end event.
    fn is_link_end(event: &Event) -> bool {
        matches!(event, Event::End(TagEnd::Link))
    }

    /// Prints Markdown for normal target.
    fn print_normal(&self, state: State<'a>) -> PrintOut<'a> {
        let events = self.events.iter().cloned();
        MdPrinter::raw_print_resume(events, state)
    }

    /// Prints Markdown for broken link.
    fn print_broken_link(&self, state: State<'a>) -> PrintOut<'a> {
        let contents = &self.events[1..(self.events.len() - 1)];
        let contents = contents.iter().cloned();
        let PrintOut { text, state } = MdPrinter::raw_print_resume(contents, state);

        let Event::Start(link_bgn) = &self.events[0] else {
            unreachable!("{}", msg::EVENT_SHOULD_BE_BROKEN_LINK)
        };

        let Tag::Link { link_type, id, .. } = link_bgn else {
            unreachable!("{}", msg::EVENT_SHOULD_BE_BROKEN_LINK)
        };

        let text = match link_type {
            LinkType::ShortcutUnknown => format!("[{text}]"),
            LinkType::CollapsedUnknown => format!("[{text}][]"),
            LinkType::ReferenceUnknown => format!("[{text}][{id}]"),
            _ => unreachable!("{}", msg::EVENT_SHOULD_BE_BROKEN_LINK),
        };

        PrintOut { text, state }
    }

    /// Reads normal partition.
    fn read_normal<I>(events: &mut Peekable<I>) -> Vec<Event<'a>>
    where
        I: Iterator<Item = Event<'a>>,
    {
        let mut ret = Vec::new();
        while let Some(peek) = events.peek() {
            if Self::is_not_normal(peek) {
                break;
            }

            let next = events.next().unwrap();
            ret.push(next);
        }

        ret
    }

    /// Reads broken link partition.
    fn read_broken_link<I>(events: &mut I) -> Vec<Event<'a>>
    where
        I: Iterator<Item = Event<'a>>,
    {
        let mut ret = Vec::new();

        for next in events.by_ref() {
            let end = Self::is_link_end(&next);
            ret.push(next);

            if end {
                break;
            }
        }

        ret
    }
}

/// Markdown type.
pub(crate) enum MdType {
    /// Normal.
    Normal,
    /// Broken Link.
    BrokenLink,
}
