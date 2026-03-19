//! Item IDs.

use crate::util::syn_tool::*;

/// Name of head macros.
const HEAD: &str = "head";

/// Name of body macros.
const BODY: &str = "body";

/// Name of top macros.
const TOP: &str = "top";

/// Name of subs macros.
const SUBS: &str = "subs";

/// Name of definitions macros.
const DEFS: &str = "defs";

/// Name of all mcacros.
const ALL: &str = "all";

/// Name of base modules.
const BASE: &str = "base";

/// Name of side modules.
const SIDE: &str = "side";

/// Name of sub module.
const SUB: &str = "sub";

/// Returns ID of head macros.
pub(crate) fn head() -> syn::Ident {
    ns::id(HEAD)
}

/// Returns ID of body macros.
pub(crate) fn body() -> syn::Ident {
    ns::id(BODY)
}

/// Returns ID of top macros.
pub(crate) fn top() -> syn::Ident {
    ns::id(TOP)
}

/// Returns ID of subs macros.
pub(crate) fn subs() -> syn::Ident {
    ns::id(SUBS)
}

/// Returns ID of definitions macros.
pub(crate) fn defs() -> syn::Ident {
    ns::id(DEFS)
}

/// Returns ID of all macros.
pub(crate) fn all() -> syn::Ident {
    ns::id(ALL)
}

/// Returns ID of base modules.
pub(crate) fn base() -> syn::Ident {
    ns::id(BASE)
}

/// Returns ID of side modules.
pub(crate) fn side() -> syn::Ident {
    ns::id(SIDE)
}

/// Returns ID of sub modules.
pub(crate) fn sub() -> syn::Ident {
    ns::id(SUB)
}
