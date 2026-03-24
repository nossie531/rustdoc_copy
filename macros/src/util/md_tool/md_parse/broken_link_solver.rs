//! Provider of [`BrokenLinkSolver`]

use pulldown_cmark::{BrokenLink, BrokenLinkCallback, CowStr};

/// Broken link solver.
///
/// This is necessary to prevent links being converted to text even if there
/// is no corresponding definition for the link reference. In other words,
/// without this, invalid link will translate like `\[link\]`.
pub(crate) struct BrokenLinkSolver();

impl<'a> BrokenLinkCallback<'a> for BrokenLinkSolver {
    fn handle_broken_link(&mut self, _link: BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)> {
        Some((CowStr::Borrowed(""), CowStr::Borrowed("")))
    }
}
