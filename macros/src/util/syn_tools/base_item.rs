//! Provider of [`BaseItem`].

use crate::util::syn_tools::side_items::*;
use crate::util::syn_tools::*;
use std::{iter, mem};

/// Root item.
#[repr(transparent)]
pub(crate) struct BaseItem(syn::Item);

impl BaseItem {
    /// Creates a new instance.
    pub fn new(base: syn::Item) -> Self {
        Self(base)
    }

    /// Returns `true` if this item is documentable.
    pub fn is_documentable(&self) -> bool {
        #[rustfmt::skip]
        return matches!(
            self.0,
            | syn::Item::Const(_)
            | syn::Item::Enum(_)
            | syn::Item::Fn(_)
            | syn::Item::Impl(_)
            | syn::Item::Macro(_)
            | syn::Item::Mod(_)
            | syn::Item::Static(_)
            | syn::Item::Struct(_)
            | syn::Item::Trait(_)
            | syn::Item::Type(_)
        );
    }

    /// Returns `true` if this item has side items.
    pub fn has_sides(&self) -> bool {
        #[rustfmt::skip]
        return matches!(
            self.0,
            | syn::Item::Enum(_)
            | syn::Item::Impl(_)
            | syn::Item::Struct(_)
            | syn::Item::Trait(_)
        );
    }

    /// Returns item.
    pub fn get(&self) -> &syn::Item {
        &self.0
    }

    /// Returns attributes.
    pub fn attrs(&self) -> &Vec<syn::Attribute> {
        const EMPTY: &Vec<syn::Attribute> = &vec![];
        match &self.0 {
            syn::Item::Const(x) => &x.attrs,
            syn::Item::Fn(x) => &x.attrs,
            syn::Item::Macro(x) => &x.attrs,
            syn::Item::Mod(x) => &x.attrs,
            syn::Item::Static(x) => &x.attrs,
            syn::Item::Type(x) => &x.attrs,
            syn::Item::Enum(x) => &x.attrs,
            syn::Item::Impl(x) => &x.attrs,
            syn::Item::Struct(x) => &x.attrs,
            syn::Item::Trait(x) => &x.attrs,
            _ => EMPTY,
        }
    }

    /// Returns side items of item.
    pub fn sides(&self) -> impl Iterator<Item = &dyn SideItem> {
        return match &self.0 {
            syn::Item::Enum(x) => convert(for_enum_variants(x)),
            syn::Item::Impl(x) => convert(for_impl_items(x)),
            syn::Item::Trait(x) => convert(for_trait_items(x)),
            syn::Item::Struct(x) => convert(for_struct_fields(x)),
            _ => Box::new(iter::empty()),
        };

        fn for_enum_variants(item: &syn::ItemEnum) -> impl Iterator<Item = &VariantNt> {
            item.variants.iter().map(AsRef::as_ref)
        }

        fn for_impl_items(item: &syn::ItemImpl) -> impl Iterator<Item = &ImplItemNt> {
            item.items.iter().filter_map(ImplItemNt::try_as_ref)
        }

        fn for_trait_items(item: &syn::ItemTrait) -> impl Iterator<Item = &TraitItemNt> {
            item.items.iter().filter_map(TraitItemNt::try_as_ref)
        }

        fn for_struct_fields(item: &syn::ItemStruct) -> impl Iterator<Item = &FieldNt> {
            item.fields.iter().map(AsRef::as_ref)
        }

        fn convert<'a>(
            iter: impl 'a + Iterator<Item = &'a (impl 'a + SideItem)>,
        ) -> Box<dyn 'a + Iterator<Item = &'a (dyn 'a + SideItem)>> {
            Box::new(iter.map(|x| x as &dyn SideItem))
        }
    }

    /// Returns mutable side items of item.
    pub fn sides_mut(&mut self) -> impl Iterator<Item = &mut dyn SideItem> {
        return match &mut self.0 {
            syn::Item::Enum(x) => convert(for_enum_variants(x)),
            syn::Item::Impl(x) => convert(for_impl_items(x)),
            syn::Item::Trait(x) => convert(for_trait_items(x)),
            syn::Item::Struct(x) => convert(for_struct_fields(x)),
            _ => Box::new(iter::empty()),
        };

        fn for_enum_variants(item: &mut syn::ItemEnum) -> impl Iterator<Item = &mut VariantNt> {
            item.variants.iter_mut().map(AsMut::as_mut)
        }

        fn for_impl_items(item: &mut syn::ItemImpl) -> impl Iterator<Item = &mut ImplItemNt> {
            item.items.iter_mut().filter_map(ImplItemNt::try_as_mut)
        }

        fn for_trait_items(item: &mut syn::ItemTrait) -> impl Iterator<Item = &mut TraitItemNt> {
            item.items.iter_mut().filter_map(TraitItemNt::try_as_mut)
        }

        fn for_struct_fields(item: &mut syn::ItemStruct) -> impl Iterator<Item = &mut FieldNt> {
            item.fields.iter_mut().map(AsMut::as_mut)
        }

        fn convert<'a>(
            iter: impl 'a + Iterator<Item = &'a mut (impl 'a + SideItem)>,
        ) -> Box<dyn 'a + Iterator<Item = &'a mut (dyn 'a + SideItem)>> {
            Box::new(iter.map(|x| x as &mut dyn SideItem))
        }
    }

    /// Sets attributes.
    pub fn set_attrs(&mut self, value: Vec<syn::Attribute>) {
        match &mut self.0 {
            syn::Item::Const(x) => x.attrs = value,
            syn::Item::Fn(x) => x.attrs = value,
            syn::Item::Macro(x) => x.attrs = value,
            syn::Item::Mod(x) => x.attrs = value,
            syn::Item::Static(x) => x.attrs = value,
            syn::Item::Type(x) => x.attrs = value,
            syn::Item::Enum(x) => x.attrs = value,
            syn::Item::Impl(x) => x.attrs = value,
            syn::Item::Struct(x) => x.attrs = value,
            syn::Item::Trait(x) => x.attrs = value,
            _ => {}
        }
    }
}

impl AsRef<BaseItem> for syn::Item {
    fn as_ref(&self) -> &BaseItem {
        unsafe { mem::transmute(self) }
    }
}
