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
pub(crate) fn print_item_doc(item: &syn::Item, mod_id: &syn::Ident) -> TokenStream {
    let mod_content = &match item {
        syn::Item::Const(x) => print_single(x, mod_id),
        syn::Item::Fn(x) => print_single(x, mod_id),
        syn::Item::Macro(x) => print_single(x, mod_id),
        syn::Item::Mod(x) => print_single(x, mod_id),
        syn::Item::Static(x) => print_single(x, mod_id),
        syn::Item::Type(x) => print_single(x, mod_id),
        syn::Item::Enum(x) => print_base_and_sides(x, mod_id),
        syn::Item::Impl(x) => print_base_and_sides(x, mod_id),
        syn::Item::Struct(x) => print_base_and_sides(x, mod_id),
        syn::Item::Trait(x) => print_base_and_sides(x, mod_id),
        _ => panic!("{}", msg::ITEM_SHOULD_EXCLUDED),
    };

    templates::module(mod_id, mod_content)
}

/// Returns single item tokens.
fn print_single<T>(item: &T, mod_id: &syn::Ident) -> TokenStream
where 
    T: AbstItem,
{
    let md = &doc_attr::read(item.attrs());
    let chunk = &DocChunk::parse(md);
    let path = &ns::path([mod_id]);
    chunk.print(path)
}

/// Returns base and side tokens.
fn print_base_and_sides<T>(item: &T, mod_id: &syn::Ident) -> TokenStream
where 
    T: AbstItem,
{
    let base_macros = print_base(item, mod_id);
    let side_macros = print_sides(item, mod_id);
    return templates::base_and_sides(base_macros, side_macros);

    fn print_sides<T>(item: &T, mod_id: &syn::Ident) -> TokenStream
    where 
        T: AbstItem,
    {
        let mut ret = TokenStream::new();
        for side in item.sides() {
            let id = &side.0;
            let attrs = &side.1;
            ret.extend(print_side(mod_id, id, attrs));
        }

        ret        
    }
}

/// Returns base module tokens.
fn print_base<T>(item: &T, mod_id: &syn::Ident) -> TokenStream
where
    T: AbstItem,
{
    let md = &doc_attr::read(item.attrs());
    let chunk = &DocChunk::parse(md);
    let base_path = &ns::path([mod_id, &ids::base()]);
    chunk.print(base_path)
}

/// Returns side module tokens.
fn print_side(mod_id: &syn::Ident, id: &syn::Ident, attrs: &[Attribute]) -> TokenStream {
    let md = &doc_attr::read(attrs);
    let chunk = &DocChunk::parse(md);
    let path = &ns::path([mod_id, &ids::side(), id]);
    let doc_tokens = &chunk.print(path);
    templates::module(id, doc_tokens)
}
