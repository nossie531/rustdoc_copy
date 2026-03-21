//! Messages.

/// Message that assert event should be broken link.
pub const EVENT_SHOULD_BE_BROKEN_LINK: &str = "Event must be broken link.";

/// Message that assert format to string should be success.
pub const FORMAT_SHOULD_SUCCESS: &str = "Format to string should be success.";

/// Message that detect event nest level is underflowed.
pub const EVENT_NEST_LV_UNDERFLOW: &str = "Event nest level is underflowed.";

/// Message that detect event nest level is overflowed.
pub const EVENT_NEST_LV_OVERFLOW: &str = "Event nest level is overflowed.";

/// Message that detect fragment not found.
pub const FRAGMENT_NOT_FOUND: &str = "Fragment not found.";

/// Message that detect Markdown print failes.
pub const MD_PRINT_FAIL: &str = "Markdown print failed.";

/// Message that detect unexpected item for `doc_share` attribute.
pub const UNEXPECTED_ITEM: &str = "`doc_share` can not use this item type.";

/// Message that expects moderate number of same titles.
pub const MODERATE_SAME_TITLES: &str = "Too many same titles.";
