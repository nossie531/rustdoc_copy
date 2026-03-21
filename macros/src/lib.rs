//! Procedual macros for crate `rustdoc_copy`.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

mod custom;
mod doc;
mod msg;
mod print;
mod util;
use proc_macro as pm;
use proc_macro2::TokenStream;

/// Share documentation comment as given name module.
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
    let attr = TokenStream::from(attr);
    let body = TokenStream::from(body);
    let ret = custom::DocShare::translate(attr, body);
    pm::TokenStream::from(ret)
}

/// Include Markdown file as given name module.
#[proc_macro]
pub fn doc_file(input: pm::TokenStream) -> pm::TokenStream {
    let input = TokenStream::from(input);
    let ret = custom::DocFile::translate(input);
    pm::TokenStream::from(ret)
}
