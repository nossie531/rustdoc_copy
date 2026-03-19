//! Markdown tools.
//!
//! This module is working with the [`pulldown_cmark`] and
//! [`pulldown_cmark_to_cmark`] crate.

pub(crate) mod md_parse;
pub(crate) mod md_print;
pub(crate) use funcs::*;
pub(crate) use md_outline::*;
pub(crate) use md_path::*;
pub(crate) use url_event::*;

mod funcs;
mod md_outline;
mod md_path;
mod url_event;
