//! Provider of [`proc_doc_file`].

use proc_macro as pm;

/// Process [`doc_file`](crate::doc_file).
pub(crate) fn proc_doc_file(input: pm::TokenStream) -> pm::TokenStream {
    #[cfg(not(feature = "doc_on"))]
    return doc_off::proc_doc_file(input);

    #[cfg(feature = "doc_on")]
    return doc_on::proc_doc_file(input);
}

/// Module for feature flag `doc_on` is OFF.
#[cfg(not(feature = "doc_on"))]
mod doc_off {
    use proc_macro as pm;

    /// Body of [`proc_doc_file`](super::proc_doc_file) at case `doc_on` is OFF.
    pub(crate) fn proc_doc_file(_input: pm::TokenStream) -> pm::TokenStream {
        pm::TokenStream::new()
    }
}

/// Module for feature flag `doc_on` is ON.
#[cfg(feature = "doc_on")]
mod doc_on {
    use crate::nodes::*;
    use proc_macro as pm;
    use proc_macro2::TokenStream;

    /// Body of [`proc_doc_file`](super::proc_doc_file) at case `doc_on` is ON.
    #[cfg(feature = "doc_on")]
    pub(crate) fn proc_doc_file(input: pm::TokenStream) -> pm::TokenStream {
        let input = TokenStream::from(input);
        let ret = DocFileNode::translate(input);
        pm::TokenStream::from(ret)
    }
}
