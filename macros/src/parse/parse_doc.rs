//! Provider of [`parse_doc()`].

use crate::doc::*;
use crate::parse::*;
use crate::util::md_tool::md_parse::*;
use crate::util::md_tool::*;
use std::collections::HashSet;

/// Returns document chunk from given Markdown text.
pub(crate) fn parse_doc(md: &str) -> DocChunk<'_> {
    let parser = MdParser::parse(md);
    let mut ret = build_doc_tree(parser);
    assign_chunk_ids(&mut ret);
    ret
}

/// Build document chunk tree from parser.
fn build_doc_tree(parser: MdParser) -> DocChunk {
    let ret = DocChunk::new_empty_root(parser.defs);
    let buf = &mut EventBuffer::new();
    let mut curr = ret.clone();

    for event in parser.events {
        if let MdOutline::Heading(lv) = MdOutline::get(&event) {
            let child = DocChunk::new_empty_chunk(lv);
            let parent = &mut find_parent(&curr, &child);
            curr = parent.append_chunk(child);
        }

        if let Some(block) = buf.input(event) {
            curr.append_block(block);
        }
    }

    ret
}

/// Returns the parent where the given chunk should be added.
fn find_parent<'a>(start: &DocChunk<'a>, target: &DocChunk<'a>) -> DocChunk<'a> {
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
