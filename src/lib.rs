//! Rustdoc comment copy helper.
//!
//! _The author of this crate is not good at English._  
//! _Forgive me if the document is hard to read._

#![cfg_attr(feature = "doc_on", doc = doc::sub::core_items::all!())]
#![cfg_attr(feature = "doc_on", doc = doc::sub::examples::all!())]
#![cfg_attr(feature = "doc_on", doc = doc::sub::tips::all!())]
#![cfg_attr(feature = "doc_on", doc = doc::sub::trouble_shooting::all!())]
#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

doc_file!(doc, "README.md#");

pub mod prelude;
pub use rustdoc_copy_macro;
pub use rustdoc_copy_macro::doc_on_only;
pub use rustdoc_copy_macro::doc_share;

#[cfg(feature = "doc_on")]
pub mod docs;

mod macros;

#[doc(hidden)]
#[cfg(feature = "doc_on")]
#[path = "../tests_compile_fail/mod.rs"]
mod tests_compile_fail;

