//! Provider of [`MdPrinter`].

use crate::util::md_tool::md_print::*;
use crate::*;
use pulldown_cmark::Event;
use pulldown_cmark_to_cmark as cm2cm;
use pulldown_cmark_to_cmark::{Error, State};

/// Markdown printer.
pub(crate) struct MdPrinter {}

impl MdPrinter {
    /// Stringify Markdown events.
    ///
    /// This method performs a small preprocessing before printing.
    /// See documentation of [`MdSplitter`] for detail.
    pub fn print<'a>(events: impl Iterator<Item = Event<'a>>) -> String {
        let mut ret = String::new();
        let mut state = State::default();
        let splitter = MdSplitter::new(events);
        for partition in splitter {
            let out = partition.print(state);
            ret += &out.text;
            state = out.state;
        }

        ret
    }

    /// Stringify Markdown events with raw mode.
    ///
    /// This method is almost a proxy to [`cmark`](cm2cm::cmark).
    pub fn raw_print<'a>(events: impl Iterator<Item = Event<'a>>) -> String {
        let mut text = String::new();
        let result = cm2cm::cmark(events, &mut text);
        let _state = Self::process_print_result(result);
        text
    }

    /// Resume stringification of Markdown events with raw mode.
    ///
    /// This method is almost a proxy to [`cmark_resume`](cm2cm::cmark_resume).
    pub fn raw_print_resume<'a>(
        events: impl Iterator<Item = Event<'a>>,
        state: State<'a>,
    ) -> PrintOut<'a> {
        let mut text = String::new();
        let result = cm2cm::cmark_resume(events, &mut text, Some(state));
        let state = Self::process_print_result(result);
        PrintOut { state, text }
    }

    /// Process result of stringification of Markdown.
    fn process_print_result<'a>(result: Result<State<'a>, Error>) -> State<'a> {
        match result {
            Ok(state) => state,
            Err(Error::FormatFailed(_)) => unreachable!("{}", msg::FORMAT_SHOULD_SUCCESS),
            Err(Error::UnexpectedEvent) => panic!("{}", msg::MD_PRINT_FAIL),
        }
    }
}
