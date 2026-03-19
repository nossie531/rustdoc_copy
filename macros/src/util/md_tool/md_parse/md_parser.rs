use crate::util::md_tool::md_parse::*;
use pulldown_cmark::{Options, Parser, RefDefs};
use std::collections::HashMap;

/// Base Markdown parser.
type BaseParser<'a> = Parser<'a, MyBlcb>;

/// Markdown parser.
pub(crate) struct MdParser<'a> {
    /// Markdown events.
    pub events: BaseParser<'a>,
    /// URL definitions map.
    pub defs: HashMap<String, String>,
}

impl<'a> MdParser<'a> {
    /// Parse text.
    pub fn parse(text: &'a str) -> Self {
        let options = Self::options();
        let callback = Some(MyBlcb());
        let events = Parser::new_with_broken_link_callback(text, options, callback);
        let defs = Self::to_hash_map(events.reference_definitions());
        Self { events, defs }
    }

    /// Returns map of Markdown definitions part.
    pub(crate) fn to_hash_map(rd: &RefDefs) -> HashMap<String, String> {
        let mut ret = HashMap::new();
        for (key, def) in rd.iter() {
            ret.insert(key.to_string(), def.dest.to_string());
        }

        ret
    }

    /// Markdown parsing options.
    ///
    /// Rustdoc uses [CommonMark][cm] syntax with several [extensions][rd].
    ///
    /// [cm]: https://commonmark.org/
    /// [rd]: https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html#markdown
    fn options() -> Options {
        let mut ret = Options::empty();
        ret.insert(Options::ENABLE_TABLES);
        ret.insert(Options::ENABLE_FOOTNOTES);
        ret.insert(Options::ENABLE_TASKLISTS);
        ret.insert(Options::ENABLE_STRIKETHROUGH);
        ret.insert(Options::ENABLE_SMART_PUNCTUATION);
        ret
    }
}
