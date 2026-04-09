//! Markdown events.

pub(crate) use text_event::*;
pub(crate) use url_event::*;

mod text_event;
mod url_event;
