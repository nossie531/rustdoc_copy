use rustdoc_copy::prelude::*;

#[test]
fn simple() {
    #[doc_on_only]
    #[doc = doc::all!()]
    #[allow(unused)]
    fn alias() {}

    /// Some document.
    #[doc_share(doc)]
    #[allow(unused)]
    fn target_item() {}
}
