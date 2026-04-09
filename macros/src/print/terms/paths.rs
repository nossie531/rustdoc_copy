//! Item paths.

use crate::print::terms::*;
use crate::util::syn_tools::*;

/// Returns path of head macro.
pub(crate) fn head_macro() -> syn::Path {
    ns::path([&ids::head()])
}

/// Returns path of body macro.
pub(crate) fn body_macro() -> syn::Path {
    ns::path([&ids::body()])
}

/// Returns path of top macro.
pub(crate) fn top_macro() -> syn::Path {
    ns::path([&ids::top()])
}

/// Returns path of subs macro.
pub(crate) fn subs_macro() -> syn::Path {
    ns::path([&ids::subs()])
}

/// Returns path of all in section macro.
pub(crate) fn sub_all_macro(ident: &syn::Ident) -> syn::Path {
    ns::path([&ids::sub(), ident, &ids::all()])
}

/// Returns path of definitions macro.
pub(crate) fn defs_macro() -> syn::Path {
    ns::path([&ids::defs()])
}

/// Returns path of base module.
pub(crate) fn base(root: &syn::Ident) -> syn::Path {
    ns::path([root, &ids::base()])
}

/// Returns path of side item module.
pub(crate) fn side_item(root: &syn::Ident, id: &syn::Ident) -> syn::Path {
    ns::path([root, &ids::side(), id])
}
