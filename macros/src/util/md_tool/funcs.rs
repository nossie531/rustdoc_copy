//! Utility functions for Markdown.

use crate::util::md_tool::md_print::*;
use crate::util::md_tool::*;
use pulldown_cmark::{Event, HeadingLevel, LinkType, Tag, TagEnd};

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

/// Retruns event with embeding definitions style.
pub(crate) fn embed_link<'a>(event: &Event<'a>) -> Event<'a> {
    return match event {
        Event::Start(Tag::Link { .. }) => {
            let url_event = &mut UrlEvent::try_new(event).unwrap();
            url_event.link_type = to_embeding_type(&url_event.link_type);
            url_event.to_link()
        }
        Event::Start(Tag::Image { .. }) => {
            let url_event = &mut UrlEvent::try_new(event).unwrap();
            url_event.link_type = to_embeding_type(&url_event.link_type);
            url_event.to_image()
        }
        _ => event.clone(),
    };

    fn to_embeding_type(link_type: &LinkType) -> LinkType {
        match link_type {
            LinkType::Reference => LinkType::Inline,
            LinkType::Collapsed => LinkType::Inline,
            LinkType::Shortcut => LinkType::Inline,
            _ => *link_type,
        }
    }
}
