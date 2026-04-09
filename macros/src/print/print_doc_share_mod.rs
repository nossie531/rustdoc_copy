//! Provider of [`print_doc_share_mod`].

use crate::doc_parts::*;
use crate::print::terms::*;
use crate::util::syn_tools::*;
use crate::*;
use proc_macro2::TokenStream;

/// Returns tokens of given [`DocShareMod`].
pub(crate) fn print_doc_share_mod(doc_share_mod: &DocShareMod) -> TokenStream {
    let mod_id = doc_share_mod.mod_id();
    let base_item = doc_share_mod.item().as_ref() as &BaseItem;
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
    let md = &doc_attr::read_attrs(item.attrs());
    let path = &ns::path([mod_id]);
    let chunk = &parse::parse_rs_doc(item.get(), md);
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
    let md = &doc_attr::read_attrs(item.attrs());
    let path = &paths::base(mod_id);
    let chunk = &parse::parse_rs_doc(item.get(), md);
    let chunk_mod = &DocChunkMod::new_root(path, chunk);
    print::print_doc_chunk_mod(chunk_mod)
}

/// Returns side module tokens.
fn print_side(mod_id: &syn::Ident, item: &BaseItem) -> TokenStream {
    let mut contents = TokenStream::new();

    for (i, ref side) in item.sides().enumerate() {
        let md = &doc_attr::read_attrs(side.attrs());
        let id = &ns::id(&side.id(i));
        let path = &paths::side_item(mod_id, id);
        let chunk = &parse::parse_rs_doc(item.get(), md);
        let chunk_mod = &DocChunkMod::new_root(path, chunk);
        contents.extend(print::print_doc_chunk_mod(chunk_mod));
    }

    templates::module(&ids::side(), &contents)
}
