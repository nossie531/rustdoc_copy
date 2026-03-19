//! Item paths.

use crate::doc::terms::*;
use crate::util::syn_tool::*;

/// Path of head macro.
pub(crate) fn head_macro() -> syn::Path {
    ns::path([&ids::head()])
}

/// Path of body macro.
pub(crate) fn body_macro() -> syn::Path {
    ns::path([&ids::body()])
}

/// Path of top macro.
pub(crate) fn top_macro() -> syn::Path {
    ns::path([&ids::top()])
}

/// Path of subs macro.
pub(crate) fn subs_macro() -> syn::Path {
    ns::path([&ids::subs()])
}

/// Path of all in section macro.
pub(crate) fn sub_all_macro(ident: syn::Ident) -> syn::Path {
    ns::path([&ids::sub(), &ident, &ids::all()])
}

/// Path of definitions macro.
pub(crate) fn defs_macro() -> syn::Path {
    ns::path([&ids::defs()])
}
