//! Provider of [`print_doc_share_item`].

use crate::{print::doc_share_self, util::syn_tools::*};
use proc_macro2::TokenStream;
use quote::ToTokens;

/// Returns tokens of given [`Item`](syn::Item).
///
/// This modifies documentation comment part. More specifically,
/// the part is `doc_share::Self` path in the API links. And it
/// is replaced with the actual item name.
pub(crate) fn print_doc_share_item(item: &syn::Item) -> TokenStream {
    // Replace item document.
    let mut item = BaseItem::new(item.clone());
    let old_item_attrs = item.attrs().iter();
    let new_item_attrs = old_item_attrs.map(replace_doc_text);
    item.set_attrs(new_item_attrs.collect());

    // Replace side item document.
    for side in item.sides_mut() {
        let old_side_attrs = side.attrs().iter();
        let new_side_attrs = old_side_attrs.map(replace_doc_text);
        side.set_attrs(new_side_attrs.collect());
    }

    item.get().to_token_stream()
}

/// Returns attribute with replaced documentation text.
fn replace_doc_text(attr: &syn::Attribute) -> syn::Attribute {
    let Some(ref text) = doc_attr::read_attr(attr) else {
        return attr.clone();
    };

    let text = &doc_share_self::replace_text(text);
    doc_attr::to_attr(text)
}
