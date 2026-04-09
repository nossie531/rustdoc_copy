//! `doc` attribute value reading tools.

use syn::parse_quote;

/// Returns `doc` attribute values.
pub(crate) fn read_attr(attr: &syn::Attribute) -> Option<String> {
    Some(trim_doc_first_sp(&doc_val(attr)?).to_string())
}

/// Returns concatinated `doc` attribute values.
pub(crate) fn read_attrs(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .filter_map(read_attr)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns `doc` attribute.
pub(crate) fn to_attr(text: &str) -> syn::Attribute {
    parse_quote!(#[doc = #text])
}

/// Returns text without first SP.
///
/// # Notes
///
/// Most documentation comments start with single SP.
///
/// ```no_run
/// /// <--- Remove this single SP!
/// fn my_func() {
///     println!("`my_func` is called.");
/// }
/// ```
fn trim_doc_first_sp(text: &str) -> &str {
    text.strip_prefix(" ").unwrap_or(text)
}

/// Returns text of `doc` attribute from some attribute.
///
/// # Notes
///
/// In following cases, only first case returns [`Some`] text.
///
/// - `#[doc = "This is doc comment"]`
/// - `#[doc("This is doc comment")]`
/// - `#[doc = "This is" + "doc comment"]`
/// - `#[doc = 42]`
/// - `#[not_doc = "This is doc comment."]`
///
/// (Others returns [`None`], because it is illegal format.)
fn doc_val(attr: &syn::Attribute) -> Option<String> {
    let syn::Meta::NameValue(ref nv) = attr.meta else {
        return None;
    };

    let syn::Expr::Lit(ref value) = nv.value else {
        return None;
    };

    let syn::Lit::Str(ref value) = value.lit else {
        return None;
    };

    nv.path.is_ident("doc").then(|| value.value())
}
