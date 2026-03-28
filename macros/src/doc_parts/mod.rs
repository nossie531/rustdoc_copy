//! Document system.

pub(crate) mod chunk_id;
pub(crate) use doc_chunk::*;
pub(crate) use doc_chunk_mod::*;
pub(crate) use doc_file_mod::*;
pub(crate) use doc_meta::*;
pub(crate) use doc_share_mod::*;

mod doc_chunk;
mod doc_chunk_mod;
mod doc_file_mod;
mod doc_meta;
mod doc_share_mod;
