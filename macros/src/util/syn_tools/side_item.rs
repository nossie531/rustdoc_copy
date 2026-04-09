//! Provider of [`SideItem`].

/// Side item.
pub(crate) trait SideItem {
    /// Returns ID.
    fn id(&self, index: usize) -> String;
    /// Returns attributes.
    fn attrs(&self) -> &Vec<syn::Attribute>;
    /// Sets attributes.
    fn set_attrs(&mut self, value: Vec<syn::Attribute>);
}
