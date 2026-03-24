//! Utility functions for Markdown.

use crate::util::md_tool::md_print::*;
use pulldown_cmark::{Event, HeadingLevel, Tag, TagEnd};

/// Returns plain text of Markdown events.
pub(crate) fn text<'a>(events: impl Iterator<Item = Event<'a>>) -> String {
    MdPrinter::raw_print(events.filter(|x| matches!(x, Event::Text(_))))
}

/// Returns event with added heading level.
///
/// # Panics
///
/// Panics if level is overflowed (MIN: 1, MAX: 6).
pub(crate) fn add_level<'a>(event: Event<'a>, delta: i8) -> Event<'a> {
    return match event {
        Event::Start(Tag::Heading { level, .. }) => {
            let new_heading = new_heading(add_lv(level, delta));
            Event::Start(new_heading)
        }
        Event::End(TagEnd::Heading(level)) => {
            let new_end_heading = new_end_heading(add_lv(level, delta));
            Event::End(new_end_heading)
        }
        _ => event,
    };

    fn add_lv(level: HeadingLevel, delta: i8) -> HeadingLevel {
        let ret = (level as u8).checked_add_signed(delta);
        let ret = ret.and_then(|x| (x as usize).try_into().ok());
        ret.expect("Lv overflowed.")
    }

    fn new_heading<'a>(level: HeadingLevel) -> Tag<'a> {
        Tag::Heading {
            level,
            id: None,
            classes: vec![],
            attrs: vec![],
        }
    }

    fn new_end_heading(level: HeadingLevel) -> TagEnd {
        TagEnd::Heading(level)
    }
}
