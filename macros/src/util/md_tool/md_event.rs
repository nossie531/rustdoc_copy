//! Provider of [`MdEvent`].

use pulldown_cmark::{CowStr, Event, LinkType, Tag};
use std::mem;

/// Markdown event.
#[repr(transparent)]
pub(crate) struct MdEvent<'a>(Event<'a>);

impl<'a> MdEvent<'a> {
    /// Returns link event if it exists.
    pub fn as_link(event: &Event<'a>) -> Option<UrlEvent<'a>> {
        match event {
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => Some(UrlEvent {
                link_type: *link_type,
                dest_url: dest_url.clone(),
                title: title.clone(),
                id: id.clone(),
            }),
            _ => None,
        }
    }

    /// Returns image event if it exists.
    pub fn as_image(event: &Event<'a>) -> Option<UrlEvent<'a>> {
        match event {
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) => Some(UrlEvent {
                link_type: *link_type,
                dest_url: dest_url.clone(),
                title: title.clone(),
                id: id.clone(),
            }),
            _ => None,
        }
    }
}

impl<'a> AsRef<MdEvent<'a>> for Event<'a> {
    fn as_ref(&self) -> &MdEvent<'a> {
        unsafe { mem::transmute(self) }
    }
}

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
