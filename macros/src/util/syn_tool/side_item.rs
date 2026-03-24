//! Provider of [`SideItem`].

use syn::Attribute;

/// Side item.
pub(crate) struct SideItem<'a> {
    /// ID.
    id: String,
    /// Attributes.
    attrs: &'a Vec<Attribute>,
}

impl<'a> SideItem<'a> {
    /// Creates a new instance.
    pub fn new(id: String, attrs: &'a Vec<Attribute>) -> Self {
        Self { id, attrs }
    }

    /// Returns ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns attributes.
    pub fn attrs(&self) -> &Vec<Attribute> {
        self.attrs
    }
}
