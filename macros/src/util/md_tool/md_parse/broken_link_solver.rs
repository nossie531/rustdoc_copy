//! Provider of [`MyBlcb`]

use pulldown_cmark::{BrokenLink, BrokenLinkCallback, CowStr};

/// Broken link solver.
///
/// All broken link keep as link (Do not translate like `\[link\]`).
pub(crate) struct MyBlcb();

impl<'a> BrokenLinkCallback<'a> for MyBlcb {
    fn handle_broken_link(&mut self, _link: BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)> {
        Some((CowStr::Borrowed(""), CowStr::Borrowed("")))
    }
}
