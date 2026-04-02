//! Procedual macros for crate `rustdoc_copy`.
//!
//! _The author of this crate is not good at English._  
//! _Forgive me if the document is hard to read._

mod doc_parts;
mod entries;
mod msg;
mod nodes;
mod parse;
mod print;
mod util;
use proc_macro as pm;

/// Includes Markdown file as given name module.
#[proc_macro]
pub fn doc_file(input: pm::TokenStream) -> pm::TokenStream {
    entries::proc_doc_file(input)
}

/// Shares documentation comment as given name module.
///
/// # Examples
///
/// ```
/// use rustdoc_copy::prelude::*;
///
/// /// My function.
/// #[doc_share(doc)]
/// pub fn my_func() {
///     println("`my_func` is called.");
/// }
///
/// #[doc = doc::all!()]
/// pub fn my_func_alias() {
///     my_func();
/// }
/// ```
#[proc_macro_attribute]
pub fn doc_share(attr: pm::TokenStream, body: pm::TokenStream) -> pm::TokenStream {
    entries::proc_doc_share(attr, body)
}

/// Disables documentations if `doc_on` feature flag is OFF.
///
/// # Build acceleration
///
/// This crate is bit heavy (As of 2026, default build takes several seconds
/// on the author's environment). Because it includes parser crates for Rust
/// and Markdown text. And sadly, `Cargo.toml` has `dev-dependencies` but no
/// `doc-dependencies`. As a result, outside of documentation, this crate is
/// nothing more than a heavy burden.
///
/// For this solution, `doc_on` feature flag and [`doc_on_only`](Self)
/// attribute are used in combination. First, if `doc_on` is OFF, most items
/// are replaced with items that have only their entry points. This speed up
/// the build process. However, since document copy is not generated, its
/// importing part cause compiation errors. Here, to avoid this error,
/// [`doc_on_only`](Self) generates empty document when `doc_on` is OFF.
///
/// # Build settings
///
/// This attribute is often used with settings like followings.
///
/// -&nbsp;`Cargo.toml`
///
/// ```text
/// [package]
/// name = "example"
/// version = "0.1.0"
/// edition = "2024"
///
/// [dependencies]
/// rustdoc_copy = { version = "0.2.0", default-features = false }
///
/// [features]
/// doc_on = ["rustdoc_copy/doc_on"]
///
/// [package.metadata.docs.rs]
/// all-features = true
/// ```
///
/// -&nbsp;`.vscode/settings.json`
/// 
/// ```text
/// {"rust-analyzer.cargo.allFeatures": true}
/// ```
///
/// This example roughly does the following.
/// 
/// - Define a feature flag named `doc_on` in the crate.
/// - Propagates it to the feature flag of the same name in this crate.
/// - Configures to enable the `doc_on` flag in [`docs.rs`](https://docs.rs/).
/// - Configures to enable the `doc_on` flag in IDE (VS Code).
///
/// In this case, following command generate rustdoc locally.
///
/// ```text
/// cargo doc --features doc_on
/// ```
///
/// # Examples
///
/// ```
/// use rustdoc_copy::prelude::*;
///
/// /// My function.
/// #[doc_share(doc)]
/// pub fn my_func() {
///     println("`my_func` is called.");
/// }
///
/// #[doc_on_only]
/// #[doc = doc::all!()]
/// pub fn my_func_alias() {
///     my_func();
/// }
/// ```
#[proc_macro_attribute]
pub fn doc_on_only(attr: pm::TokenStream, body: pm::TokenStream) -> pm::TokenStream {
    entries::proc_doc_on_only(attr, body)
}
