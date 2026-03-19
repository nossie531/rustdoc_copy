//! Markdown parsing tools.

pub(crate) use event_buffer::*;
pub(crate) use md_parser::*;

mod broken_link_solver;
mod event_buffer;
mod md_parser;
use broken_link_solver::*;
