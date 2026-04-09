//! Provider of [`MdPath`].

/// Markdown path.
pub(crate) struct MdPath<'a> {
    /// File path.
    path: &'a str,
    /// Fragment key,
    key: Option<&'a str>,
}

impl<'a> MdPath<'a> {
    /// Anonymous root key.
    pub const ANONYMOUS_ROOT: &'static str = "";

    /// Returns a file path.
    pub fn path(&self) -> &'a str {
        self.path
    }

    /// Returns a fragment key.
    pub fn key(&self) -> Option<&'a str> {
        self.key
    }
}

impl<'a> From<&'a str> for MdPath<'a> {
    fn from(value: &'a str) -> Self {
        let values = &mut value.trim().splitn(2, '#');
        Self {
            path: values.next().unwrap(),
            key: values.next(),
        }
    }
}
