//! Provider of [`DocChunk`].

use crate::doc::*;
use crate::util::md_tool::md_parse::*;
use crate::util::md_tool::*;
use crate::util::*;
use pulldown_cmark::{CowStr, Event, LinkType, Tag};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

/// Copy guard definition key.
const COPY_GUARD: &str = "!copy_guard";

/// Markdown document chunk.
///
/// Document chunks are created from one of the following.
///
/// - Root of document
/// - Section of document
#[derive(Clone, Debug)]
pub(crate) struct DocChunk<'a>(Rc<RefCell<DocChunkBody<'a>>>);

/// Body of [`DocChunk`].
#[derive(Default, Debug)]
pub(crate) struct DocChunkBody<'a> {
    /// Heading level.
    level: u8,
    /// Markdown Fragment ID of this chunk.
    md_id: Option<String>,
    /// Rust ID of this chunk.
    rs_id: Option<String>,
    /// Events of head.
    head_events: Vec<Event<'a>>,
    /// Events of body.
    body_events: Vec<Event<'a>>,
    /// Definitions.
    defs: Rc<HashMap<String, String>>,
    /// Parent chunk.
    parent: Option<Weak<RefCell<DocChunkBody<'a>>>>,
    /// Child chunks.
    chunks: Vec<DocChunk<'a>>,
}

impl<'a> DocChunk<'a> {
    /// Creates a new instance from Markdown.
    pub fn new(md: &'a str) -> Self {
        let parser = MdParser::parse(md);
        let mut ret = Self::build_doc_tree(parser);
        ret.assign_chunk_ids();
        ret
    }

    /// Returns borrowed body.
    pub fn borrow(&self) -> Ref<'_, DocChunkBody<'a>> {
        self.0.borrow()
    }

    /// Returns mutable borrowed body.
    pub fn borrow_mut(&self) -> RefMut<'_, DocChunkBody<'a>> {
        self.0.borrow_mut()
    }

    /// Returns document chank matched given key.
    pub fn extract(&mut self, key: Option<&str>) -> Option<Self> {
        let chunk = &mut find(self, key)?;
        adjust_root_heading(chunk, matches!(key, Some(MdPath::ANONYMOUS_ROOT)));
        adjust_tree_level(chunk, level_delta(chunk, key));
        return Some(chunk.clone());

        // Find chunk.
        fn find<'a>(chunk: &DocChunk<'a>, key: Option<&str>) -> Option<DocChunk<'a>> {
            if is_hit(chunk, key) {
                Some(chunk.clone())
            } else {
                chunk.borrow().chunks().find_map(|x| find(&x, key))
            }
        }

        // Returns `true` if key is hit at chunk.
        fn is_hit(chunk: &DocChunk, key: Option<&str>) -> bool {
            let Some(key) = key else { return true };
            let root_hit = chunk.borrow().level() == 1 && key == MdPath::ANONYMOUS_ROOT;
            let normal_hit = chunk.borrow().md_id.as_deref() == Some(key);
            root_hit || normal_hit
        }

        // Returns delta of heading levels.
        fn level_delta(chunk: &DocChunk, key: Option<&str>) -> i8 {
            match key {
                None => 0,
                Some(MdPath::ANONYMOUS_ROOT) => -1,
                _ => 1 - (chunk.borrow().level() as i8),
            }
        }

        // Adjust root heading.
        fn adjust_root_heading(chunk: &mut DocChunk, no_heading: bool) {
            if no_heading {
                chunk.borrow_mut().head_events.clear();
            }
        }

        // Adjust heading level in sub tree.
        fn adjust_tree_level(chunk: &mut DocChunk, delta: i8) {
            adjust_chunk_level(chunk, delta);
            for mut chunk in chunk.borrow().chunks.iter().cloned() {
                adjust_tree_level(&mut chunk, delta);
            }
        }

        // Adjust heading level of document chunk.
        fn adjust_chunk_level(chunk: &mut DocChunk, delta: i8) {
            let chunk = &mut chunk.borrow_mut();
            let old_events = chunk.head_events.drain(..);
            let new_events = old_events.map(|x| md_tool::add_level(x, delta));
            let imports = new_events.collect::<Vec<_>>();
            chunk.head_events.extend(imports);
        }
    }

    /// Creates a new empty root.
    fn new_empty_root(defs: HashMap<String, String>) -> Self {
        Self(Rc::new(RefCell::new(DocChunkBody {
            defs: Rc::new(defs),
            ..Default::default()
        })))
    }

    /// Creates a new empty chunk.
    fn new_empty_chunk(level: u8) -> Self {
        Self(Rc::new(RefCell::new(DocChunkBody {
            level,
            ..Default::default()
        })))
    }

    /// Build document tree from parser.
    fn build_doc_tree(parser: MdParser) -> DocChunk {
        let ret = DocChunk::new_empty_root(parser.defs);
        let buf = &mut EventBuffer::new();
        let mut curr = ret.clone();

        for event in parser.events {
            if let MdOutline::Heading(lv) = MdOutline::get(&event) {
                let child = DocChunk::new_empty_chunk(lv);
                let parent = &mut curr.seek_parent(&child);
                curr = parent.append_chunk(child);
            }

            let Some(block) = buf.input(event) else {
                continue;
            };

            curr.append_block(block);
        }

        ret
    }

    /// Returns the target where the given chunk should be added.
    fn seek_parent(&self, child: &DocChunk) -> Self {
        let mut curr = self.clone();

        while curr.borrow().level() >= child.borrow().level() {
            let curr_ref = curr.borrow();
            let parent = curr_ref.parent.as_ref().cloned();
            let Some(parent) = parent else { break };
            drop(curr_ref);
            curr = DocChunk(parent.upgrade().unwrap());
        }

        curr
    }

    /// Appends block.
    fn append_block(&mut self, block: Vec<Event<'a>>) {
        let this = &mut self.borrow_mut();
        if this.head_events.is_empty() {
            this.head_events = block;
        } else {
            this.body_events.extend(block);
        }
    }

    /// Appends child chunk.
    fn append_chunk(&mut self, child: DocChunk<'a>) -> Self {
        let this = &mut self.borrow_mut();
        this.chunks.push(child.clone());
        child.borrow_mut().defs = this.defs.clone();
        child.borrow_mut().parent = Some(Rc::downgrade(&self.0));
        child
    }

    /// Assigns unique ID to each chunk.
    fn assign_chunk_ids(&mut self) {
        let md_ids = &mut HashSet::new();
        let rs_ids = &mut HashSet::new();
        traverse(self.clone(), md_ids, rs_ids);

        // Traverse chunks.
        fn traverse(chunk: DocChunk, md_ids: &mut HashSet<String>, rs_ids: &mut HashSet<String>) {
            let this = &mut chunk.borrow_mut();

            if !this.title().is_empty() {
                let md_id = chunk_id::md_id(&this.title(), md_ids);
                let rs_id = chunk_id::rs_id(&this.title(), rs_ids);
                this.md_id = Some(md_id.clone());
                this.rs_id = Some(rs_id.clone());
                md_ids.insert(md_id);
                rs_ids.insert(rs_id);
            }

            // Traverse child chunks.
            let rs_ids = &mut HashSet::new();
            for chunk in this.chunks.iter().cloned() {
                traverse(chunk, md_ids, rs_ids);
            }
        }
    }
}

impl<'a> DocChunkBody<'a> {
    /// Returns `true` if this chunk is root.
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Returns the level
    pub fn level(&self) -> u8 {
        self.level
    }

    /// Returns Rust ID of this chunk.
    pub fn rust_id(&self) -> Option<&str> {
        self.rs_id.as_deref()
    }

    /// Returns title.
    pub fn title(&self) -> String {
        md_tool::text(&mut self.head_events())
    }

    /// Returns events of heading.
    pub fn head_events(&self) -> impl Iterator<Item = Event<'a>> {
        self.head_events.iter().map(|x| self.adjust_url_event(x))
    }

    /// Returns events of body.
    pub fn body_events(&self) -> impl Iterator<Item = Event<'a>> {
        self.body_events.iter().map(|x| self.adjust_url_event(x))
    }

    /// Returns copy guard URL root.
    pub fn copy_guard(&self) -> Option<&str> {
        self.defs.get(COPY_GUARD).map(|x| x.as_str())
    }

    /// Returns definitions blocks.
    pub fn defs(&self) -> impl Iterator<Item = (&str, &str)> {
        self.defs
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .filter(move |(_, url)| !self.guards_url(url))
    }

    /// Returns chunks.
    pub fn chunks(&self) -> impl Iterator<Item = DocChunk<'a>> {
        self.chunks.iter().cloned()
    }

    /// Returns `true` if given URL is guarded.
    pub fn guards_url(&self, url: &str) -> bool {
        self.copy_guard().is_some_and(|x| url.starts_with(x))
    }

    /// Adjust URL event.
    fn adjust_url_event<'x>(&self, event: &Event<'x>) -> Event<'x> {
        let event = &self.adjust_url_event_by_copy_guard(event);
        md_tool::embed_url(event)
    }

    /// Adjust URL event by copy guard.    
    fn adjust_url_event_by_copy_guard<'x>(&self, event: &Event<'x>) -> Event<'x> {
        let Some(url_event) = UrlEvent::try_new_link(event) else {
            return event.clone();
        };

        if !self.guards_url(&url_event.dest_url) {
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
}
