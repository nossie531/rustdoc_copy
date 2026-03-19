use pulldown_cmark_to_cmark::State;

/// Print output.
pub(crate) struct PrintOut<'a> {
    /// Markdown text.
    pub text: String,
    /// Context state for resume print.
    pub state: State<'a>,
}
