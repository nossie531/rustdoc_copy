//! Provider of [`DocChunkMod`].

use crate::doc::*;

/// Document chunk module.
pub(crate) struct DocChunkMod<'a, 'd> {
    /// Document module path.
    mod_path: &'a syn::Path,
    /// Document chunk.
    chunk: &'a DocChunk<'d>,
    /// `true` if this is root.
    is_root: bool,
}

impl<'a, 'd> DocChunkMod<'a, 'd> {
    /// Returns a new instance for root.
    pub fn new_root(mod_path: &'a syn::Path, chunk: &'a DocChunk<'d>) -> Self {
        Self {
            chunk,
            mod_path,
            is_root: true,
        }
    }

    /// Returns a new instance for root.
    pub fn new_sub(mod_path: &'a syn::Path, chunk: &'a DocChunk<'d>) -> Self {
        Self {
            chunk,
            mod_path,
            is_root: false,
        }
    }

    /// Returns `true` if this is root.
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    /// Returns document module path.
    pub fn mod_path(&self) -> &'a syn::Path {
        self.mod_path
    }

    /// Returns document module ID.
    pub fn mod_id(&self) -> &'a syn::Ident {
        &self.mod_path.segments.last().unwrap().ident
    }

    /// Returns document chunk.
    pub fn chunk(&self) -> &'a DocChunk<'d> {
        self.chunk
    }
}
