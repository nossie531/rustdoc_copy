//! Crate's macros.

#![doc(hidden)]

/// Include Markdown file as given name module.
///
/// ## File path
///
/// Argument file path is relative path from the crate root directory.
///
/// Note, this behavior is different from [`include_str`] which uses
/// relative path from calling file.
///
/// ## Fragment key
///
/// You can specify a fragment key after the file path.
///
/// - If the key is not specified, all document is included.
/// - If the key is `#`, first level 1 section is included.
/// - If the key is `#some-id`, the fragment section is included.
///
/// ## Heading Level adjust
///
/// Heading level adjust works with [fragment key](#fragment-key).\
/// This adjust target heading of fragment key to level 1.\
/// And adjust levels of subsequent headings relative to it.
///
/// ## Link copy guard
///
/// `!copy_guard` is a special reference definition with this crate.\
/// All URLs of links in Markdown, including the guard path, are disabled.
///
/// This is primarily intended for use in `README.md`, like the following.
/// This ensures that the links in rustdoc are valid even if `docs.rs`
/// is unavailable. Additionally, `cargo doc` will report warning if
/// `my_func` is removed.
///
/// ```text
/// # my_crate
///
/// Welcome to my crate.
///
/// This crate has only one function [`my_func`].
///
/// [!copy_guard]: https://docs.rs/my_crate/latest/
/// [`my_func`]: https://docs.rs/my_crate/latest/my_crate/fn.my_func.html
/// ```
///
/// # Examples
///
/// \- `src/lib.rs`
///
/// ```rust, ignore
/// //! Welcome to my crate API document.
/// #![doc = doc::sub::examples::all!()]
///
/// doc_file!(doc, "README.md#");
///
/// use rustdoc_copy::prelude::*;
///
/// pub fn some_func() -> i32 {
///     42
/// }
/// ```
///
/// \- `README.md`
///
/// ````text
/// # my_crate
///
/// This crate is ...
///
/// ## Examples
///
/// ```rust
/// use my_crate::some_func;
///
/// assert_eq!(some_func(), 42);
/// ```
/// ````
#[macro_export]
macro_rules! doc_file {
    ($id:ident, $path:literal) => {
        $crate::rustdoc_copy_macro::doc_file!($id, $path);
    };
}
