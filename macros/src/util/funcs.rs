//! Utility functions.

/// Returns `true` if the text has given circumfix.
pub(crate) fn surrounds_with(text: &str, circumfix: &str) -> bool {
    text.starts_with(circumfix) && text.ends_with(circumfix)
}

/// Returns a string slice with the optional circumfix removed.
///
/// This is substitute for nightly only [`str::trim_prefix`].
pub(crate) fn trim_circumfix<'a>(text: &'a str, circumfix: &'a str) -> &'a str {
    match surrounds_with(text, circumfix) {
        false => text,
        true => trim_prefix(trim_suffix(text, circumfix), circumfix),
    }
}

/// Returns a string slice with the optional prefix removed.
///
/// This is substitute for nightly only [`str::trim_prefix`].
pub(crate) fn trim_prefix<'a>(text: &'a str, prefix: &'a str) -> &'a str {
    text.strip_prefix(prefix).unwrap_or(text)
}

/// Returns a string slice with the optional suffix removed.
///
/// This is substitute for nightly only [`str::trim_suffix`].
pub(crate) fn trim_suffix<'a>(text: &'a str, prefix: &'a str) -> &'a str {
    text.strip_suffix(prefix).unwrap_or(text)
}
