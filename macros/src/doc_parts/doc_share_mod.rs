//! Provider of [`DocShareMod`].

/// Mmodule parts for [`doc_share`][crate::doc_share].
pub(crate) struct DocShareMod<'a> {
    /// ID of document module.
    mod_id: &'a syn::Ident,
    /// Target item.
    item: &'a syn::Item,
}

impl<'a> DocShareMod<'a> {
    /// Creates a new instance.
    pub fn new(mod_id: &'a syn::Ident, item: &'a syn::Item) -> Self {
        Self { mod_id, item }
    }

    /// Returns ID of document module.
    pub fn mod_id(&self) -> &'a syn::Ident {
        self.mod_id
    }

    /// Returns target item.
    pub fn item(&self) -> &'a syn::Item {
        self.item
    }
}
