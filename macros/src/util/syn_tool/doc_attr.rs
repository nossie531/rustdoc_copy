//! `doc` attribute value reading tools.

use syn::Attribute;

/// Returns concatinated `doc` attribute values.
pub(crate) fn read(attrs: &[Attribute]) -> String {
    let docs = attrs.iter().filter_map(doc_val);
    docs.map(|x| trim_doc_first_sp(&x).to_string())
        .collect::<Vec<_>>()
        .join("\n")
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
/// - `#[doc = "This is doc comment"`]
/// - `#[doc("This is doc comment")]`
/// - `#[doc = "This is" + "doc comment"]`
/// - `#[doc = 42]`
/// - `#[not_doc = "This is doc comment."]`
///
/// (Others returns [`None`], because it is illegal format.)
fn doc_val(attr: &Attribute) -> Option<String> {
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
