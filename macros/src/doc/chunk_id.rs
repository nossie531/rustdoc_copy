//! Chunk ID solver.

use crate::util::*;
use crate::*;
use std::collections::HashSet;
use std::iter;

/// Returns Markdown fragment ID from given title.
pub(crate) fn md_id(title: &str, used_ids: &HashSet<String>) -> String {
    let raw_id = naming::to_fragment_id(title);
    let fst_opt = iter::once(raw_id.to_string());
    let alt_opts = (1..u32::MAX).map(|i| format!("{raw_id}-{i}"));
    let all_opts = &mut fst_opt.chain(alt_opts);
    let ret = all_opts.find(|x| !used_ids.contains(x));
    ret.expect(msg::MODERATE_SAME_TITLES)
}

/// Returns Rust ID from given title.
pub(crate) fn rs_id(title: &str, used_ids: &HashSet<String>) -> String {
    let raw_id = naming::to_lsc_rust_id(title);
    let fst_opt = iter::once(raw_id.to_string());
    let alt_opts = (1..u32::MAX).map(|i| format!("{raw_id}_{i}"));
    let all_opts = &mut fst_opt.chain(alt_opts);
    let ret = all_opts.find(|x| !used_ids.contains(x));
    ret.expect(msg::MODERATE_SAME_TITLES)
}
