//! Tools for parsing document chunk.

use crate::doc::*;
use crate::util::md_tool::md_parse::*;
use crate::util::md_tool::*;

/// Returns document chunk from Markdown text.
pub(crate) fn parse_doc(md: &str) -> DocChunk<'_> {
    let parser = MdParser::parse(md);
    let mut ret = build_doc_tree(parser);
    ret.assign_chunk_ids();
    ret
}

/// Build document tree from parser.
fn build_doc_tree(parser: MdParser) -> DocChunk {
    let ret = DocChunk::new_root(parser.defs);
    let buf = &mut EventBuffer::new();
    let mut curr = ret.clone();

    for event in parser.events {
        if let MdOutline::Heading(lv) = MdOutline::get(&event) {
            let child = DocChunk::new_chunk(lv);
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
