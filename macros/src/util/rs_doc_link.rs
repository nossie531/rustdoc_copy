//! Operations for Rustdoc API link.

use crate::util::syn_tool::*;
use crate::util::*;
use crate::*;

/// Returns `true` if given URL contains `Self`.
///
/// # How to detect API path or not?
///
/// To determine whether a URL contains `Self` of Rust, first we must
/// determine whether it is a Rust API path or not. Because, for example,
/// `Self` as a plain string could also appear in URL query parameters.
///
/// ## Current strategy
///
/// If the string does not contain character '`/`', the string is treated
/// as a Rust path. Because '`/`' is not included in Rust API path, but is
/// often included in external URLs. Note that this approach may mistake
/// about strings that are neither external URLs nor Rust paths.
/// Fortunately, such patterns are highly unlikely in practice.
///
/// ## Future TODO?
///
/// For more careful handling, we should also verify that the string
/// is a valid Rust path. However, currently doing that would result
/// in some rather clunky code.
///
/// Followings are current situation.
///
/// - [`syn::parse_str`] can determine whether a string is a valid Rust path.
/// - It calls [`from_str`] of [`proc_macro::TokenStream`].
/// - It calls [`from_str`] of [`proc_macro2::TokenStream`].
/// - It generates compilation error! automatically about invalid tokens.
/// - This behavior acts only in procedural macro context.
/// - This behavior can prevent by <code>[proc_macro2]::fallback::force</code>.
/// - However, this feature is not documented and seemes unofficial.
///
/// [`from_str`]: std::str::FromStr::from_str
pub(crate) fn has_self(value: &str) -> bool {
    if value.contains("/") {
        return false;
    }

    let value = trim_backtick(value);
    RsTokens::new(value).find(RsToken::is_self).is_some()
}

/// Returns the path with `Self` replaced by given item.
pub(crate) fn replace_self(value: &str, self_item: Option<&syn::Item>) -> String {
    if !has_self(value) {
        return value.to_string();
    }

    let Some(ref api_id) = self_item.and_then(api_id) else {
        return value.to_string();
    };

    let value = trim_backtick(value);
    let old_tokens = RsTokens::new(value);
    let new_tokens = old_tokens.map(|ref x| replace_self_token(x, api_id));
    new_tokens.collect()
}

/// Returns given text with backticks trimed.
fn trim_backtick(value: &str) -> &str {
    util::trim_circumfix(value, "`")
}

/// Returns the token string with `Self` replaced by given ID.
fn replace_self_token(token: &RsToken, id: &syn::Ident) -> String {
    if token.is_self() {
        id.to_string()
    } else {
        token.code().to_string()
    }
}

/// Returns API ID.
fn api_id(item: &syn::Item) -> Option<syn::Ident> {
    match item {
        syn::Item::Const(x) => Some(x.ident.clone()),
        syn::Item::Enum(x) => Some(x.ident.clone()),
        syn::Item::Fn(x) => Some(x.sig.ident.clone()),
        syn::Item::Macro(x) => x.ident.clone(),
        syn::Item::Mod(x) => Some(x.ident.clone()),
        syn::Item::Static(x) => Some(x.ident.clone()),
        syn::Item::Struct(x) => Some(x.ident.clone()),
        syn::Item::Trait(x) => Some(x.ident.clone()),
        syn::Item::Type(x) => Some(x.ident.clone()),
        syn::Item::Impl(x) => type_api_id(&x.self_ty),
        _ => None,
    }
}

/// Returns type API ID.
///
/// Note: Some primitive supports aliases in rustdoc.
fn type_api_id(ty: &syn::Type) -> Option<syn::Ident> {
    match ty {
        syn::Type::Array(_) => Some(ns::id("array")),
        syn::Type::BareFn(_) => Some(ns::id("fn")),
        syn::Type::Ptr(_) => Some(ns::id("pointer")),
        syn::Type::Reference(_) => Some(ns::id("reference")),
        syn::Type::Slice(_) => Some(ns::id("slice")),
        syn::Type::Tuple(_) => Some(ns::id("tuple")),
        syn::Type::Path(x) => x.path.get_ident().cloned(),
        _ => None,
    }
}
