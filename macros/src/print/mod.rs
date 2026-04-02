//! Document printing system.

#![cfg(feature = "doc_on")]

pub(crate) mod terms;
pub(crate) use print_doc_chunk_mod::*;
pub(crate) use print_doc_file_mod::*;
pub(crate) use print_doc_share_mod::*;

mod link_adjuster;
mod print_doc_chunk_mod;
mod print_doc_file_mod;
mod print_doc_share_mod;
use link_adjuster::*;
