//! Tests that should be compile errors.

/// File not found.
///
/// ```compile_fail
/// use rustdoc_copy::prelude::*;
/// doc_file!(doc, "md/not_found.md");
/// ```
fn _file_not_found() {
    // nop.
}

/// Fragment ID not found.
///
/// ```compile_fail
/// use rustdoc_copy::prelude::*;
/// doc_file!(doc, "md/simple.md#not_found");
/// ```
fn _id_not_found() {
    // nop.
}

/// Document title not found.
///
/// ```compile_fail
/// use rustdoc_copy::prelude::*;
/// doc_file!(doc, "md/untitled_doc.md#");
/// ```
fn _doc_title_not_found() {
    // nop.
}
