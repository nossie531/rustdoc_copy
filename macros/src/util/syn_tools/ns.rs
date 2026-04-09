//! Naming system.

use proc_macro2::Span;
use syn::punctuated::Punctuated;

/// Retruns a new identifier.
pub(crate) fn id(id: &str) -> syn::Ident {
    syn::Ident::new(id, Span::mixed_site())
}

/// Returns path from IDs.
pub(crate) fn path<'a>(idents: impl IntoIterator<Item = &'a syn::Ident>) -> syn::Path {
    let mut ret = new_path();
    ret.segments.extend(segments(idents));
    ret
}

/// Returns path with new segment.
pub(crate) fn path_with<'a>(
    path: &syn::Path,
    idents: impl IntoIterator<Item = &'a syn::Ident>,
) -> syn::Path {
    let mut ret = path.clone();
    ret.segments.extend(segments(idents));
    ret
}

/// Returns new path.
fn new_path() -> syn::Path {
    syn::Path {
        leading_colon: None,
        segments: Punctuated::new(),
    }
}

/// Returns new segments.
fn segments<'a>(
    idents: impl IntoIterator<Item = &'a syn::Ident>,
) -> impl IntoIterator<Item = syn::PathSegment> {
    idents.into_iter().map(|ident| syn::PathSegment {
        ident: ident.clone(),
        arguments: syn::PathArguments::None,
    })
}
