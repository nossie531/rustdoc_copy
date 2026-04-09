//! Provider of [`TraitItemNt`].

use crate::util::syn_tools::*;
use crate::*;
use std::mem;

/// Newtype of [`syn::TraitItem`].
#[repr(transparent)]
pub(crate) struct TraitItemNt(syn::TraitItem);

impl TraitItemNt {
    /// Returns a new immutable reference if given item is target item.
    pub fn try_as_ref(value: &syn::TraitItem) -> Option<&Self> {
        Self::is_target(value).then_some(value.as_ref())
    }

    /// Returns a new mutable reference if given item is target item.
    pub fn try_as_mut(value: &mut syn::TraitItem) -> Option<&mut Self> {
        Self::is_target(value).then_some(value.as_mut())
    }

    /// Returns `true` if given item is target item.
    fn is_target(value: &syn::TraitItem) -> bool {
        matches!(
            value,
            syn::TraitItem::Fn(_) | syn::TraitItem::Type(_) | syn::TraitItem::Const(_)
        )
    }
}

impl SideItem for TraitItemNt {
    fn id(&self, _index: usize) -> String {
        match &self.0 {
            syn::TraitItem::Fn(x) => x.sig.ident.to_string(),
            syn::TraitItem::Type(x) => x.ident.to_string(),
            syn::TraitItem::Const(x) => x.ident.to_string(),
            _ => panic!("{}", msg::INVALID_ITEM_TYPE),
        }
    }

    fn attrs(&self) -> &Vec<syn::Attribute> {
        match &self.0 {
            syn::TraitItem::Fn(x) => &x.attrs,
            syn::TraitItem::Type(x) => &x.attrs,
            syn::TraitItem::Const(x) => &x.attrs,
            _ => panic!("{}", msg::INVALID_ITEM_TYPE),
        }
    }

    fn set_attrs(&mut self, value: Vec<syn::Attribute>) {
        match &mut self.0 {
            syn::TraitItem::Fn(x) => x.attrs = value,
            syn::TraitItem::Type(x) => x.attrs = value,
            syn::TraitItem::Const(x) => x.attrs = value,
            _ => panic!("{}", msg::INVALID_ITEM_TYPE),
        }
    }
}

impl AsRef<TraitItemNt> for syn::TraitItem {
    fn as_ref(&self) -> &TraitItemNt {
        unsafe { mem::transmute(self) }
    }
}

impl AsMut<TraitItemNt> for syn::TraitItem {
    fn as_mut(&mut self) -> &mut TraitItemNt {
        unsafe { mem::transmute(self) }
    }
}
