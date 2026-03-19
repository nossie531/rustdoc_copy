//! Provider of [`SkipAttr`].

use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

/// Item tokens after attributes.
///
/// This type is used for adjusting error report span. Normally, the tokens of
/// an item includes tokens of their attributes. However, we sometimes want to
/// exclude tokens of their attributes from error span. To achieve this, This
/// type reverts the parsed item to token stream, and skips attributes, and
/// re-parses rest token stream as item again.
pub struct SkipAttr {
    item: syn::Item,
}

impl SkipAttr {
    /// Reparse item and skip its attributes.
    pub fn reparse(item: &syn::Item) -> syn::Item {
        let tokens = item.to_token_stream();
        syn::parse2::<Self>(tokens).unwrap().item
    }
}

impl Parse for SkipAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ = syn::Attribute::parse_outer(input);
        let item = syn::Item::parse(input)?;
        Ok(Self { item })
    }
}
