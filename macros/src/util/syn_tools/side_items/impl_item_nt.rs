//! Provider of [`ImplItemNt`].

use crate::util::syn_tools::*;
use crate::*;
use std::mem;

/// Newtype of [`syn::ImplItem`].
#[repr(transparent)]
pub(crate) struct ImplItemNt(syn::ImplItem);

impl ImplItemNt {
    /// Returns a new immutable reference if given item is target item.
    pub fn try_as_ref(value: &syn::ImplItem) -> Option<&Self> {
        Self::is_target(value).then_some(value.as_ref())
    }

    /// Returns a new mutable reference if given item is target item.
    pub fn try_as_mut(value: &mut syn::ImplItem) -> Option<&mut Self> {
        Self::is_target(value).then_some(value.as_mut())
    }

    /// Returns `true` if given item is target item.
    fn is_target(value: &syn::ImplItem) -> bool {
        matches!(
            value,
            syn::ImplItem::Fn(_) | syn::ImplItem::Type(_) | syn::ImplItem::Const(_)
        )
    }
}

impl SideItem for ImplItemNt {
    fn id(&self, _index: usize) -> String {
        match &self.0 {
            syn::ImplItem::Fn(x) => x.sig.ident.to_string(),
            syn::ImplItem::Type(x) => x.ident.to_string(),
            syn::ImplItem::Const(x) => x.ident.to_string(),
            _ => panic!("{}", msg::INVALID_ITEM_TYPE),
        }
    }

    fn attrs(&self) -> &Vec<syn::Attribute> {
        match &self.0 {
            syn::ImplItem::Fn(x) => &x.attrs,
            syn::ImplItem::Type(x) => &x.attrs,
            syn::ImplItem::Const(x) => &x.attrs,
            _ => panic!("{}", msg::INVALID_ITEM_TYPE),
        }
    }

    fn set_attrs(&mut self, value: Vec<syn::Attribute>) {
        match &mut self.0 {
            syn::ImplItem::Fn(x) => x.attrs = value,
            syn::ImplItem::Type(x) => x.attrs = value,
            syn::ImplItem::Const(x) => x.attrs = value,
            _ => panic!("{}", msg::INVALID_ITEM_TYPE),
        }
    }
}

impl AsRef<ImplItemNt> for syn::ImplItem {
    fn as_ref(&self) -> &ImplItemNt {
        unsafe { mem::transmute(self) }
    }
}

impl AsMut<ImplItemNt> for syn::ImplItem {
    fn as_mut(&mut self) -> &mut ImplItemNt {
        unsafe { mem::transmute(self) }
    }
}
