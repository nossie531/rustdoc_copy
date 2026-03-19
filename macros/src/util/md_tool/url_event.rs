use pulldown_cmark::{CowStr, Event, LinkType, Tag};

/// Markdown event including url info.
///
/// This type shortens the code related to [`Tag::Link`] and [`Tag::Image`].
/// These two variants have a bit more fields, and their contents are the same.
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
    /// Creates a new instance.
    pub fn try_new(event: &Event<'a>) -> Option<Self> {
        Self::try_new_link(event).or_else(|| Self::try_new_image(event))
    }

    /// Creates a new link instance.
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

    /// Creates a new image instance.
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
