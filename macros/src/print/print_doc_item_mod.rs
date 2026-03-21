//! Tools for parsing item document.

use crate::doc::*;
use crate::print::terms::*;
use crate::util::syn_tool::*;
use crate::*;

/// Returns document item tokens.
pub(crate) fn print_doc_item_mod(doc_item_mod: &DocItemMod) -> TokenStream {
    let mod_id = doc_item_mod.mod_id();
    let base_item = doc_item_mod.item();
    print_doc_item(mod_id, base_item)
}

/// Returns item tokens.
fn print_doc_item(mod_id: &syn::Ident, item: &BaseItem) -> TokenStream {
    let print = if !item.has_sides() {
        print_simple_doc_item
    } else {
        print_complex_doc_item
    };

    print(mod_id, item)
}

/// Returns simple document item tokens.
fn print_simple_doc_item(mod_id: &syn::Ident, item: &BaseItem) -> TokenStream {
    let md = &doc_attr::read(item.attrs());
    let path = &ns::path([mod_id]);
    let chunk = &DocChunk::new(md);
    let chunk_mod = &DocChunkMod::new_root(path, chunk);
    print::print_doc_chunk_mod(chunk_mod)
}

/// Returns complex document item tokens.
fn print_complex_doc_item(mod_id: &syn::Ident, item: &BaseItem) -> TokenStream {
    let base = print_base(mod_id, item);
    let side = print_side(mod_id, item);
    let contents = &quote::quote! {
        #base
        #side
    };

    templates::module(mod_id, contents)
}

/// Returns base module tokens.
fn print_base(mod_id: &syn::Ident, item: &BaseItem) -> TokenStream {
    let md = &doc_attr::read(item.attrs());
    let path = &paths::base(mod_id);
    let chunk = &DocChunk::new(md);
    let chunk_mod = &DocChunkMod::new_root(path, chunk);
    print::print_doc_chunk_mod(chunk_mod)
}

/// Returns side module tokens.
fn print_side(mod_id: &syn::Ident, item: &BaseItem) -> TokenStream {
    let mut contents = TokenStream::new();

    for ref side in item.sides() {
        let md = &doc_attr::read(side.attrs());
        let id = &ns::id(side.id());
        let path = &paths::side_item(mod_id, id);
        let chunk = &DocChunk::new(md);
        let chunk_mod = &DocChunkMod::new_root(path, chunk);
        contents.extend(print::print_doc_chunk_mod(chunk_mod));
    }

    templates::module(&ids::side(), &contents)
}
