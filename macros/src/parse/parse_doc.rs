//! Document parsing functions.

use crate::doc_parts::*;
use crate::parse::*;

/// Returns document chunk from Markdown file.
pub(crate) fn parse_md_file<'a>(md: &'a str) -> DocChunk<'a> {
    ParseManager::new().parse(md)
}

/// Returns document chunk from rustdoc comment.
pub(crate) fn parse_rs_doc<'a>(item: &'a syn::Item, md: &'a str) -> DocChunk<'a> {
    ParseManager::new().with_item(item).parse(md)
}
