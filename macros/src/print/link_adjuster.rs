//! Provider of [`LinkAdjuster`].

use crate::util::md_tool::*;
use crate::util::*;
use pulldown_cmark::{CowStr, Event, LinkType, Tag};

/// Markdown link adjuster.
#[derive(Default)]
pub(crate) struct LinkAdjuster {
    /// Actual target of `Self`.
    self_item: Option<syn::Item>,
    /// Copy guard URL root
    copy_guard: Option<String>,
}

impl LinkAdjuster {
    /// Creates a new instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new instance with the given self item.
    pub fn with_self_item(mut self, self_item: Option<syn::Item>) -> Self {
        self.self_item = self_item;
        self
    }

    /// Creates a new instance with the given copy guard.
    pub fn with_copy_guard(mut self, copy_guard: Option<String>) -> Self {
        self.copy_guard = copy_guard;
        self
    }

    /// Returns `true` if given URL is guarded.
    pub fn is_guard_target(&self, url: &str) -> bool {
        self.copy_guard.as_ref().is_some_and(|x| url.starts_with(x))
    }

    /// Adjusts URL event.
    pub fn adjust<'i>(&mut self, event: &Event<'i>) -> Event<'i> {
        let event = &self.adjust_for_guard(event);
        let event = &self.adjust_for_embed(event);
        let event = &self.adjust_for_self(event);
        event.clone()
    }

    /// Adjusts URL event by copy guard.
    fn adjust_for_guard<'i>(&self, event: &Event<'i>) -> Event<'i> {
        let Some(url_event) = MdEvent::as_link(event) else {
            return event.clone();
        };

        if !self.is_guard_target(&url_event.dest_url) {
            return event.clone();
        }

        Event::Start(Tag::Link {
            dest_url: CowStr::Borrowed(""),
            title: url_event.title.clone(),
            id: url_event.id.clone(),
            link_type: match url_event.link_type {
                LinkType::Reference => LinkType::ReferenceUnknown,
                LinkType::Collapsed => LinkType::CollapsedUnknown,
                LinkType::Shortcut => LinkType::ShortcutUnknown,
                LinkType::Inline => LinkType::ShortcutUnknown,
                _ => return event.clone(),
            },
        })
    }

    /// Retruns event that adjusted for URL embeding.
    fn adjust_for_embed<'i>(&self, event: &Event<'i>) -> Event<'i> {
        match event {
            Event::Start(Tag::Link { .. }) => {
                let url_event = &mut MdEvent::as_link(event).unwrap();
                url_event.link_type = Self::to_embeding_type(&url_event.link_type);
                url_event.to_link()
            }
            Event::Start(Tag::Image { .. }) => {
                let url_event = &mut MdEvent::as_image(event).unwrap();
                url_event.link_type = Self::to_embeding_type(&url_event.link_type);
                url_event.to_image()
            }
            _ => event.clone(),
        }
    }

    /// Returns event that adjusted for `Self` replacing.
    fn adjust_for_self<'i>(&self, event: &Event<'i>) -> Event<'i> {
        if !Self::is_link_with_self(event) {
            return event.clone();
        }

        let mut ret = MdEvent::as_link(event).unwrap();
        let self_item = self.self_item.as_ref();
        let new_id = rs_doc_link::replace_self(&ret.id, self_item);
        let new_url = rs_doc_link::replace_self(&ret.dest_url, self_item);
        let use_url = !new_url.is_empty();
        ret.link_type = LinkType::Inline;
        ret.dest_url = (if use_url { new_url } else { new_id }).into();
        ret.to_link()
    }

    /// Returns `true` if event is link with `Self`.
    fn is_link_with_self(event: &Event) -> bool {
        match MdEvent::as_link(event) {
            None => false,
            Some(UrlEvent { id, dest_url, .. }) => {
                let id_has_self = rs_doc_link::has_self(&id);
                let url_has_self = rs_doc_link::has_self(&dest_url);
                id_has_self || url_has_self
            }
        }
    }

    /// Returns embeding link type for given link type.
    fn to_embeding_type(link_type: &LinkType) -> LinkType {
        match link_type {
            LinkType::Reference => LinkType::Inline,
            LinkType::Collapsed => LinkType::Inline,
            LinkType::Shortcut => LinkType::Inline,
            _ => *link_type,
        }
    }
}
