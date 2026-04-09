//! Operations for adjusting `doc_share::Self`.

use crate::util::syn_tools::*;
use crate::util::*;
use crate::*;
use iter_seq_ext::prelude::*;

/// Returns `true` if given URL contains `doc_share::Self`.
///
/// # How to detect API path or not?
///
/// To determine whether a URL contains Rust path `doc_share::Self`,
/// first we must determine whether it is a Rust path or not. Because,
/// for example, URL query parameters can also contains `doc_share::Self`.
///
/// ## Current strategy
///
/// If the string does not contain character '`/`', the string is treated
/// as a Rust path. Because '`/`' is not included in Rust path, but is
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
pub(crate) fn is_target(value: &str) -> bool {
    if value.contains("/") {
        return false;
    }

    let value = trim_backtick(value);
    let tokens = RsTokens::new(value);
    IteratorSeqExt::contains(tokens, &target_path())
}

/// Returns the text replacing `doc_share::Self` with `Self`.
pub(crate) fn replace_text(value: &str) -> String {
    if !is_target(value) {
        return value.to_string();
    }

    let pat_path = &target_path();
    let alt_path = &self_path();
    let old_tokens = RsTokens::new(value);
    let new_tokens = IteratorSeqExt::replace(old_tokens, pat_path, alt_path);
    new_tokens.map(|x| x.code().to_string()).collect()
}

/// Returns the url replacing `doc_share::Self` with given item.
pub(crate) fn replace_url(value: &str, self_item: Option<&syn::Item>) -> String {
    if !is_target(value) {
        return value.to_string();
    }

    let Some(ref api_id) = self_item.and_then(api_id) else {
        return value.to_string();
    };

    let value = trim_backtick(value);
    let api_id = &api_id.to_string();
    let pat_path = &target_path();
    let alt_path = &item_path(api_id);
    let old_tokens = RsTokens::new(value);
    let new_tokens = IteratorSeqExt::replace(old_tokens, pat_path, alt_path);
    new_tokens.map(|x| x.code().to_string()).collect()
}

/// Returns `doc_share::Self` path tokens.
fn target_path() -> [RsToken<'static>; 4] {
    [
        RsToken::new("doc_share", rustc_lexer::TokenKind::Ident),
        RsToken::new(":", rustc_lexer::TokenKind::Colon),
        RsToken::new(":", rustc_lexer::TokenKind::Colon),
        RsToken::new("Self", rustc_lexer::TokenKind::Ident),
    ]
}

/// Returns `Self` path tokens.
fn self_path() -> [RsToken<'static>; 1] {
    [RsToken::new("Self", rustc_lexer::TokenKind::Ident)]
}

/// Returns item path tokens.
fn item_path<'a>(item: &'a str) -> [RsToken<'a>; 1] {
    [RsToken::new(item, rustc_lexer::TokenKind::Ident)]
}

/// Returns given text with backticks trimed.
fn trim_backtick(value: &str) -> &str {
    util::trim_circumfix(value, "`")
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
