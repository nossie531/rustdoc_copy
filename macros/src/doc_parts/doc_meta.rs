//! Provider of [`DocMeta`].

use std::collections::HashMap;

/// Document meta information.
#[derive(Default)]
pub(crate) struct DocMeta<'a> {
    /// Rust item.
    item: Option<&'a syn::Item>,
    /// Link definitions.
    defs: HashMap<String, String>,
}

impl<'a> DocMeta<'a> {
    /// Creates a new instance.
    pub fn new(item: Option<&'a syn::Item>, defs: HashMap<String, String>) -> Self {
        Self { item, defs }
    }

    /// Returns self item.
    pub fn self_item(&self) -> Option<&'a syn::Item> {
        self.item
    }

    /// Returns link definitions.
    pub fn defs(&self) -> &HashMap<String, String> {
        &self.defs
    }
}
