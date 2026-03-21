use crate::doc::*;

pub(crate) struct DocFileMod<'a> {
    mod_id: &'a syn::Ident,
    chunk: &'a DocChunk<'a>,
}

impl<'a> DocFileMod<'a> {
    pub fn new(mod_id: &'a syn::Ident, chunk: &'a DocChunk<'a>) -> Self {
        Self { mod_id, chunk }
    }

    pub fn mod_id(&self) -> &'a syn::Ident {
        self.mod_id
    }

    pub fn chunk(&self) -> &'a DocChunk<'a> {
        self.chunk
    }
}
