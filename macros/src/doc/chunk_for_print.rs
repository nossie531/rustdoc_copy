//! Provider of [`ChunkForPrint`].

use crate::doc::*;

/// Document chunk with printing hitnts.
pub(crate) struct ChunkForPrint<'a, 'b> {
    /// Document chunk.
    pub base: &'a DocChunk<'b>,
    /// Document module path.
    pub path: &'a syn::Path,
    /// Definitions print mode.
    pub defs: bool,
}

impl<'a, 'b> ChunkForPrint<'a, 'b> {
    pub fn new(base: &'a DocChunk<'b>, path: &'a syn::Path) -> Self {
        Self {
            base,
            path,
            defs: true,
        }
    }

    /// Returns a new instance with given definitions print mode.
    pub fn with_defs(mut self, value: bool) -> Self {
        self.defs = value;
        self
    }
}
