//! Provider of [`DocFileMod`].

use crate::doc::*;

/// Module parts for [`doc_file`][crate::doc_file].
pub(crate) struct DocFileMod<'a> {
    /// ID of document module.
    mod_id: &'a syn::Ident,
    /// Document chunk.
    chunk: &'a DocChunk<'a>,
}

impl<'a> DocFileMod<'a> {
    /// Creates a new instance.
    pub fn new(mod_id: &'a syn::Ident, chunk: &'a DocChunk<'a>) -> Self {
        Self { mod_id, chunk }
    }

    /// Returns ID of document module.
    pub fn mod_id(&self) -> &'a syn::Ident {
        self.mod_id
    }

    /// Returns document chunk.
    pub fn chunk(&self) -> &'a DocChunk<'a> {
        self.chunk
    }
}
