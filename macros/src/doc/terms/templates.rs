//! Item templates.

use crate::doc::terms::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Token, punctuated::Punctuated};

/// Returns module tokens.
pub(crate) fn module(id: &syn::Ident, content: &TokenStream) -> TokenStream {
    quote! {
        #[doc(hidden)]
        #[allow(nonstandard_style)]
        pub(crate) mod #id { #content }
    }
}

/// Returns `concat!` macro tokens.
pub(crate) fn concat<I>(values: I) -> TokenStream
where
    I: IntoIterator<Item = TokenStream>,
{
    let args = &mut Punctuated::<TokenStream, Token![,]>::new();
    for (i, value) in values.into_iter().enumerate() {
        if i != 0 {
            args.push(quote! { "\n\n" });
        }
        args.push(value);
    }

    quote! { concat!(#args) }
}

/// Returns document text macro tokens.
pub(crate) fn doc_macro(ident: &syn::Ident, code: &TokenStream) -> TokenStream {
    quote! {
        #[doc(hidden)]
        #[allow(unused)]
        macro_rules! #ident { () => { #code } }
        pub(crate) use #ident;
    }
}

/// Returns base and side modules tokens.
pub(crate) fn base_and_sides(base: TokenStream, sides: TokenStream) -> TokenStream {
    let base_id = &ids::base();
    let side_id = &ids::side();
    quote! {
        pub(crate) mod #base_id {
            #base
        }
        pub(crate) mod #side_id {
            #sides
        }
    }
}

