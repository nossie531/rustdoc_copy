//! Provider of [`print_doc_file_mod`].

use crate::doc::*;
use crate::util::syn_tool::*;
use crate::*;
use proc_macro2::TokenStream;

/// Returns tokens of given [`DocFileMod`].
pub(crate) fn print_doc_file_mod(doc_file_mod: &DocFileMod) -> TokenStream {
    let mod_id = doc_file_mod.mod_id();
    let mod_path = &ns::path([mod_id]);
    let chunk = doc_file_mod.chunk();
    let chunk_mod = DocChunkMod::new_root(mod_path, chunk);
    print::print_doc_chunk_mod(&chunk_mod)
}
