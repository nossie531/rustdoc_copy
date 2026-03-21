//! Item templates.

use proc_macro2::TokenStream;
use syn::Token;
use syn::punctuated::Punctuated;

/// Returns module tokens.
pub(crate) fn module(id: &syn::Ident, content: &TokenStream) -> TokenStream {
    quote::quote! {
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
            args.push(quote::quote! { "\n\n" });
        }
        args.push(value);
    }

    quote::quote! { concat!(#args) }
}

/// Returns document text macro tokens.
pub(crate) fn doc_macro(ident: &syn::Ident, code: &TokenStream) -> TokenStream {
    quote::quote! {
        #[doc(hidden)]
        #[allow(unused)]
        macro_rules! #ident { () => { #code } }
        pub(crate) use #ident;
    }
}
