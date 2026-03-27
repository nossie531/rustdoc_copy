//! Provider of [`DocChunk`].

use crate::util::md_tool::*;
use crate::util::*;
use pulldown_cmark::Event;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// Copy guard definition key.
const COPY_GUARD: &str = "!copy_guard";

/// Markdown document chunk.
///
/// Document chunks are created from one of the following.
///
/// - Root of document
/// - Section of document
#[derive(Clone)]
pub(crate) struct DocChunk<'a>(Rc<RefCell<DocChunkCore<'a>>>);

/// Core of [`DocChunk`].
#[derive(Default)]
pub(crate) struct DocChunkCore<'a> {
    /// Heading level.
    level: u8,
    /// Rust ID.
    rs_id: Option<String>,
    /// Markdown Fragment ID.
    md_id: Option<String>,
    /// Actual target of `Self` (Root only).
    self_item: Option<&'a syn::Item>,
    /// Parent chunk.
    parent: Option<Weak<RefCell<DocChunkCore<'a>>>>,
    /// Events of head.
    head_events: Vec<Event<'a>>,
    /// Events of body.
    body_events: Vec<Event<'a>>,
    /// Definitions (Root only).
    defs: Rc<HashMap<String, String>>,
    /// Child chunks.
    chunks: Vec<DocChunk<'a>>,
}

impl<'a> DocChunk<'a> {
    /// Creates a new empty root.
    pub(crate) fn new_empty_root(defs: HashMap<String, String>) -> Self {
        Self(Rc::new(RefCell::new(DocChunkCore {
            defs: Rc::new(defs),
            ..Default::default()
        })))
    }

    /// Creates a new empty chunk.
    pub(crate) fn new_empty_chunk(level: u8) -> Self {
        Self(Rc::new(RefCell::new(DocChunkCore {
            level,
            ..Default::default()
        })))
    }

    /// Creates a new instance with given self item.
    pub fn with_self_item(self, value: &'a syn::Item) -> Self {
        self.0.borrow_mut().self_item = Some(value);
        self
    }

    /// Returns borrowed body.
    pub fn borrow(&self) -> Ref<'_, DocChunkCore<'a>> {
        self.0.borrow()
    }

    /// Returns mutable borrowed body.
    pub fn borrow_mut(&self) -> RefMut<'_, DocChunkCore<'a>> {
        self.0.borrow_mut()
    }

    /// Returns parent chunk.
    pub fn parent(&self) -> Option<DocChunk<'a>> {
        self.borrow()
            .parent
            .as_ref()
            .map(|x| DocChunk(x.upgrade().unwrap()))
    }

    /// Appends block.
    pub fn append_block(&mut self, block: Vec<Event<'a>>) {
        let this = &mut self.borrow_mut();
        if this.head_events.is_empty() {
            this.head_events = block;
        } else {
            this.body_events.extend(block);
        }
    }

    /// Appends child chunk.
    pub fn append_chunk(&mut self, child: DocChunk<'a>) -> Self {
        let this = &mut self.borrow_mut();
        let child_clone = child.clone();
        let child_edit = &mut child.borrow_mut();
        child_edit.self_item = this.self_item;
        child_edit.defs = this.defs.clone();
        child_edit.parent = Some(Rc::downgrade(&self.0));
        this.chunks.push(child_clone);
        this.chunks.last().unwrap().clone()
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
            let normal_hit = chunk.borrow().md_id() == Some(key);
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
}

impl<'a> DocChunkCore<'a> {
    /// Returns `true` if this chunk is root.
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Returns the level
    pub fn level(&self) -> u8 {
        self.level
    }

    /// Returns title.
    pub fn title(&self) -> String {
        md_tool::text(self.head_events.iter().cloned())
    }

    /// Returns Rust ID.
    pub fn rs_id(&self) -> Option<&str> {
        self.rs_id.as_deref()
    }

    /// Returns Markdown Fragment ID.
    pub fn md_id(&self) -> Option<&str> {
        self.md_id.as_deref()
    }

    /// Returns Actual target of `Self`.
    pub fn self_item(&self) -> Option<&'a syn::Item> {
        self.self_item
    }

    /// Returns copy guard URL root.
    pub fn copy_guard(&self) -> Option<&str> {
        self.defs.get(COPY_GUARD).map(|x| x.as_str())
    }

    /// Returns events of heading.
    pub fn head_events(&self) -> impl Iterator<Item = Event<'a>> {
        self.head_events.iter().cloned()
    }

    /// Returns events of body.
    pub fn body_events(&self) -> impl Iterator<Item = Event<'a>> {
        self.body_events.iter().cloned()
    }

    /// Returns definitions blocks.
    pub fn defs(&self) -> impl Iterator<Item = (&str, &str)> {
        self.defs
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .filter(move |(_, url)| !self.is_guarding(url))
    }

    /// Returns chunks.
    pub fn chunks(&self) -> impl Iterator<Item = DocChunk<'a>> {
        self.chunks.iter().cloned()
    }

    /// Returns `true` if given URL is guarded.
    pub fn is_guarding(&self, url: &str) -> bool {
        self.copy_guard().is_some_and(|x| url.starts_with(x))
    }

    /// Sets Rust ID.
    pub fn set_rs_id(&mut self, value: String) {
        self.rs_id = Some(value);
    }

    /// Sets Markdown Fragment ID.
    pub fn set_md_id(&mut self, value: String) {
        self.md_id = Some(value);
    }
}
