//! Document parsing system.

#![cfg(feature = "doc_on")]

pub(crate) use parse_doc::*;

mod event_buffer;
mod parse_doc;
mod parse_manager;
use parse_manager::*;
