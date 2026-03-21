//! Tools for parsing item document.

use crate::doc::terms::*;
use crate::doc::*;
use crate::util::syn_tool::*;
use crate::*;
use syn::Attribute;

/// Returns document module tokens from item.
///
/// # Panics
///
/// Panics if item is unexpected type.
pub(crate) fn print_item(item: &syn::Item, mod_id: &syn::Ident) -> TokenStream {
    let item = &AbstItem::from_ref(item);
    let mod_tokens = &if !item.has_sides() {
        print_simple(item, mod_id)
    } else {
        print_base_and_sides(item, mod_id)
    };

    templates::module(mod_id, mod_tokens)
}

/// Returns simple item tokens.
fn print_simple(item: &AbstItem, mod_id: &syn::Ident) -> TokenStream {
    let md = &doc_attr::read(item.attrs());
    let chunk = &DocChunk::parse(md);
    let path = &ns::path([mod_id]);
    chunk.print(path)
}

/// Returns base and side tokens.
fn print_base_and_sides(item: &AbstItem, mod_id: &syn::Ident) -> TokenStream {
    let base = print_base(item, mod_id);
    let sides = print_sides(item, mod_id);
    templates::base_and_sides(base, sides)
}

/// Returns base module tokens.
fn print_base(item: &AbstItem, mod_id: &syn::Ident) -> TokenStream {
    let md = &doc_attr::read(item.attrs());
    let chunk = &DocChunk::parse(md);
    let base_path = &ns::path([mod_id, &ids::base()]);
    chunk.print(base_path)
}

/// Returns sides module tokens.
fn print_sides(item: &AbstItem, mod_id: &syn::Ident) -> TokenStream {
    let mut ret = TokenStream::new();
    for side in item.sides() {
        let id = &side.0;
        let attrs = &side.1;
        ret.extend(print_side(mod_id, id, attrs));
    }

    ret
}

/// Returns side module tokens.
fn print_side(mod_id: &syn::Ident, id: &syn::Ident, attrs: &[Attribute]) -> TokenStream {
    let md = &doc_attr::read(attrs);
    let chunk = &DocChunk::parse(md);
    let path = &ns::path([mod_id, &ids::side(), id]);
    let doc_tokens = &chunk.print(path);
    templates::module(id, doc_tokens)
}
