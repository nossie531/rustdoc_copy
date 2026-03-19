//! Syntax tools.
//!
//! This module is working with the [`syn`] crate.

pub(crate) mod doc_attr;
pub(crate) mod ns;
pub(crate) use abst_item::*;
pub(crate) use funcs::*;
pub(crate) use skip_attr::*;

mod abst_item;
mod funcs;
mod skip_attr;
