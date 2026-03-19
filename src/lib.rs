//! Rustdoc comment copy helper.
//!
//! _The author of this crate is not good at English._  
//! _Forgive me if the document is hard to read._

#![doc = doc::sub::core_items::all!()]
#![doc = doc::sub::examples::all!()]
#![doc = doc::sub::tips::all!()]
#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

doc_file!(doc, "README.md#");

pub mod prelude;
pub use rustdoc_copy_macro::doc_share;
mod macros;

#[doc(hidden)]
#[path = "../tests_compile_fail/mod.rs"]
mod tests_compile_fail;
