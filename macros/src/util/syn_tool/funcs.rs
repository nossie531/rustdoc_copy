//! Utility functions for syntax.

use proc_macro2::TokenStream;
use syn::parse::Parse;

/// Returns parsed syntax.
pub(crate) fn parse<T>(input: TokenStream) -> Result<T, TokenStream>
where
    T: Parse,
{
    syn::parse2(input).map_err(syn::Error::into_compile_error)
}

/// Returns token stream of text.
pub(crate) fn text(value: &str) -> TokenStream {
    quote::quote! { #value }
}
