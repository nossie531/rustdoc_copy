//! Syntax tools.
//!
//! This module is working with the [`syn`] crate.

pub(crate) mod doc_attr;
pub(crate) mod ns;
pub(crate) use base_item::*;
pub(crate) use funcs::*;
pub(crate) use side_item::*;
pub(crate) use skip_attr::*;

mod base_item;
mod funcs;
mod side_item;
mod skip_attr;
