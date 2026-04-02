//! Provider of [`proc_doc_share`].

use proc_macro as pm;

/// Process [`doc_share`](crate::doc_share).
pub(crate) fn proc_doc_share(attr: pm::TokenStream, body: pm::TokenStream) -> pm::TokenStream {
    #[cfg(not(feature = "doc_on"))]
    return doc_off::proc_doc_share(attr, body);

    #[cfg(feature = "doc_on")]
    return doc_on::proc_doc_share(attr, body);
}

/// Module for feature flag `doc_on` is OFF.
#[cfg(not(feature = "doc_on"))]
mod doc_off {
    use proc_macro as pm;

    /// Body of [`proc_doc_share`](super::proc_doc_share) at case `doc_on` is OFF.
    pub(crate) fn proc_doc_share(_attr: pm::TokenStream, body: pm::TokenStream) -> pm::TokenStream {
        body
    }
}

/// Module for feature flag `doc_on` is ON.
#[cfg(feature = "doc_on")]
mod doc_on {
    use crate::nodes::*;
    use proc_macro as pm;
    use proc_macro2::TokenStream;

    /// Body of [`proc_doc_share`](super::proc_doc_share) at case `doc_on` is ON.
    pub(crate) fn proc_doc_share(attr: pm::TokenStream, body: pm::TokenStream) -> pm::TokenStream {
        let attr = TokenStream::from(attr);
        let body = TokenStream::from(body);
        let ret = DocShareNode::translate(attr, body);
        pm::TokenStream::from(ret)
    }
}
