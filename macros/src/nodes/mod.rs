//! Procedural macro nodes.

#![cfg(feature = "doc_on")]

pub(crate) use doc_file_node::*;
pub(crate) use doc_share_node::*;

mod doc_file_node;
mod doc_share_node;
