//! Document system.

pub(crate) mod print_item;
pub(crate) use doc_chunk::*;

mod chunk_for_print;
mod chunk_id;
mod doc_chunk;
mod parse_doc;
mod print_doc;
mod terms;
use chunk_for_print::*;
use parse_doc::*;
use print_doc::*;
