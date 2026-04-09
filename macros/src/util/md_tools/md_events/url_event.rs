//! Provider of [`UrlEvent`].

use pulldown_cmark::{CowStr, Event, LinkType, Tag};

/// Markdown event with URL.
pub(crate) struct UrlEvent<'a> {
    /// Link type.
    link_type: LinkType,
    /// Destination URL.
    dest_url: CowStr<'a>,
    /// Link title.
    title: CowStr<'a>,
    /// ID of reference links, e.g. world in the link `[hello][world]`.
    id: CowStr<'a>,
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

    /// Creates a new instance with given link type.
    pub fn with_link_type(self, value: LinkType) -> Self {
        Self {
            link_type: value,
            ..self
        }
    }

    /// Creates a new instance with given destination URL.
    pub fn with_dest_url(self, value: &str) -> Self {
        Self {
            dest_url: value.to_string().into(),
            ..self
        }
    }

    /// Returns link type.
    pub fn link_type(&self) -> LinkType {
        self.link_type
    }

    /// Returns destination URL.
    pub fn dest_url(&self) -> CowStr<'a> {
        self.dest_url.clone()
    }

    /// Returns link title.
    pub fn title(&self) -> CowStr<'a> {
        self.title.clone()
    }

    /// Returns ID of reference links, e.g. world in the link `[hello][world]`.
    pub fn id(&self) -> CowStr<'a> {
        self.id.clone()
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
