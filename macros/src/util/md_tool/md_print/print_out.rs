//! Provider of [`PrintOut`].

use pulldown_cmark_to_cmark::State;

/// Print output and context for resume printing.
pub(crate) struct PrintOut<'a> {
    /// Markdown text.
    pub text: String,
    /// Context state for resume printing.
    pub state: State<'a>,
}
