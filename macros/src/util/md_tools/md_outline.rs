//! Provider of [`MdOutline`].

use pulldown_cmark::{Event, Tag, TagEnd};

/// Markdown outline.
pub(crate) enum MdOutline {
    /// Heading start with heading level.
    Heading(u8),
    /// Heading end.
    HeadingEnd,
    /// Body.
    Body,
}

impl MdOutline {
    /// Returns outline information of given event.
    pub fn get(event: &Event<'_>) -> Self {
        match event {
            Event::Start(Tag::Heading { level, .. }) => Self::Heading(*level as u8),
            Event::End(TagEnd::Heading { .. }) => Self::HeadingEnd,
            _ => Self::Body,
        }
    }
}
