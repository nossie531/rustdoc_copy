//! Provider of [`ParseManager`].

use crate::doc_parts::*;
use crate::parse::event_buffer::*;
use crate::util::md_tools::md_parse::*;
use crate::util::md_tools::*;
use std::collections::HashSet;

/// Document parsing manager.
pub(crate) struct ParseManager<'a> {
    item: Option<&'a syn::Item>,
}

impl<'a> ParseManager<'a> {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { item: None }
    }

    /// Creates a new instance with given syntax item.
    pub fn with_item(self, item: &'a syn::Item) -> Self {
        let item = Some(item);
        Self { item }
    }

    /// Returns a document chunk from Markdown text.
    pub fn parse(&self, md: &'a str) -> DocChunk<'a> {
        let parser = MdParser::parse(md);
        let mut ret = Self::build_doc_tree(self.item, parser);
        Self::assign_chunk_ids(&mut ret);
        ret
    }

    /// Build document chunk tree from parser.
    fn build_doc_tree(item: Option<&'a syn::Item>, parser: MdParser<'a>) -> DocChunk<'a> {
        let ret = DocChunk::new_empty_root(DocMeta::new(item, parser.defs));
        let buf = &mut EventBuffer::new();
        let mut curr = ret.clone();

        for event in parser.events {
            if let MdOutline::Heading(lv) = MdOutline::get(&event) {
                let child = DocChunk::new_empty_chunk(lv);
                let parent = &mut Self::find_parent(&curr, &child);
                curr = parent.append_chunk(child);
            }

            if let Some(block) = buf.input(event) {
                curr.append_block(block);
            }
        }

        ret
    }

    /// Returns the parent where the given chunk should be added.
    fn find_parent(start: &DocChunk<'a>, target: &DocChunk<'a>) -> DocChunk<'a> {
        let mut curr = start.clone();

        while curr.borrow().level() >= target.borrow().level() {
            curr = match curr.parent() {
                None => break,
                Some(x) => x,
            };
        }

        curr
    }

    /// Assigns unique ID to each chunk.
    fn assign_chunk_ids(root: &mut DocChunk) {
        let md_ids = &mut HashSet::new();
        let rs_ids = &mut HashSet::new();
        traverse(root.clone(), md_ids, rs_ids);

        // Traverse chunks.
        fn traverse(chunk: DocChunk, md_ids: &mut HashSet<String>, rs_ids: &mut HashSet<String>) {
            let this = &mut chunk.borrow_mut();

            if !this.title().is_empty() {
                let md_id = chunk_id::md_id(&this.title(), md_ids);
                let rs_id = chunk_id::rs_id(&this.title(), rs_ids);
                this.set_md_id(md_id.clone());
                this.set_rs_id(rs_id.clone());
                md_ids.insert(md_id);
                rs_ids.insert(rs_id);
            }

            // Traverse child chunks.
            let rs_ids = &mut HashSet::new();
            for chunk in this.chunks() {
                traverse(chunk, md_ids, rs_ids);
            }
        }
    }
}
