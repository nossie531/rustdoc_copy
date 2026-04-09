//! Provider of [`MdEvent`].

use pulldown_cmark::{CowStr, Event, LinkType, Tag};

/// Markdown event with URL.
pub(crate) struct UrlEvent<'a> {
    /// Link type.
    pub link_type: LinkType,
    /// Destination URL.
    pub dest_url: CowStr<'a>,
    /// Link title.
    pub title: CowStr<'a>,
    /// ID of reference links, e.g. world in the link `[hello][world]`.
    pub id: CowStr<'a>,
}

impl<'a> UrlEvent<'a> {
    /// Creates a new link event if it exists.
    pub fn try_new_link(event: &Event<'a>) -> Option<Self> {
        match event {
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => Some(Self {
                link_type: *link_type,
                dest_url: dest_url.clone(),
                title: title.clone(),
                id: id.clone(),
            }),
            _ => None,
        }
    }

    /// Creates a new image event if it exists.
    pub fn try_new_image(event: &Event<'a>) -> Option<Self> {
        match event {
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) => Some(Self {
                link_type: *link_type,
                dest_url: dest_url.clone(),
                title: title.clone(),
                id: id.clone(),
            }),
            _ => None,
        }
    }

    /// Returns link event tag.
    pub fn to_link(&self) -> Event<'a> {
        Event::Start(Tag::Link {
            link_type: self.link_type,
            dest_url: self.dest_url.clone(),
            title: self.title.clone(),
            id: self.id.clone(),
        })
    }

    /// Returns image event tag.
    pub fn to_image(&self) -> Event<'a> {
        Event::Start(Tag::Image {
            link_type: self.link_type,
            dest_url: self.dest_url.clone(),
            title: self.title.clone(),
            id: self.id.clone(),
        })
    }
}

/// Markdown event with text.
pub(crate) struct TextEvent<'a> {
    pub text: CowStr<'a>,
    pub is_code: bool,
}

impl<'a> TextEvent<'a> {
    /// Creates a text event if it exists.
    pub fn try_new(event: &Event<'a>) -> Option<Self> {
        match event {
            Event::Text(cow_str) => Some(Self {
                text: cow_str.clone(),
                is_code: false,
            }),
            Event::Code(cow_str) => Some(Self {
                text: cow_str.clone(),
                is_code: true,
            }),
            _ => None,
        }
    }

    /// Creates a instance with given text.
    pub fn with_text(self, value: &str) -> Self {
        Self {
            text: CowStr::Boxed(value.to_string().into_boxed_str()),
            ..self
        }
    }

    /// Retruns event.
    pub fn to_event(&self) -> Event<'a> {
        match self.is_code {
            false => Event::Text(self.text.clone()),
            true => Event::Code(self.text.clone()),
        }
    }
}
