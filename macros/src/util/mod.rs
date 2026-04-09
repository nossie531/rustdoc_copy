//! Crate's utilities.

#![cfg(feature = "doc_on")]

pub(crate) mod md_tools;
pub(crate) mod naming;
pub(crate) mod syn_tools;
pub(crate) use funcs::*;
pub(crate) use rs_tokens::*;

mod funcs;
mod rs_tokens;
