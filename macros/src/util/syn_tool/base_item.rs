//! Provider of [`RootItem`].

use crate::util::syn_tool::*;
use syn::{ItemEnum, ItemImpl, ItemStruct, ItemTrait};

/// Root item.
#[repr(transparent)]
pub(crate) struct BaseItem(syn::Item);

impl BaseItem {
    /// Creates a instance from item reference.
    pub fn from_ref(r: &syn::Item) -> &Self {
        unsafe { std::mem::transmute(r) }
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
    pub fn sides(&self) -> impl Iterator<Item = SideItem<'_>> {
        let ret: Box<dyn Iterator<Item = _>> = match &self.0 {
            syn::Item::Enum(x) => Box::new(self.enum_variants(x)),
            syn::Item::Impl(x) => Box::new(self.impl_items(x)),
            syn::Item::Struct(x) => Box::new(self.struct_fields(x)),
            syn::Item::Trait(x) => Box::new(self.trait_items(x)),
            _ => Box::new(vec![].into_iter()),
        };

        ret
    }

    /// Returns variants of `enum`.
    fn enum_variants<'a>(&'a self, item: &'a ItemEnum) -> impl Iterator<Item = SideItem<'a>> {
        item.variants.iter().map(|x| {
            let id = x.ident.to_string();
            SideItem::new(id, &x.attrs)
        })
    }

    /// Returns implement items.
    fn impl_items<'a>(&'a self, item: &'a ItemImpl) -> impl Iterator<Item = SideItem<'a>> {
        item.items.iter().filter_map(|x| {
            let (id, attrs) = match x {
                syn::ImplItem::Fn(x) => (&x.sig.ident, &x.attrs),
                syn::ImplItem::Type(x) => (&x.ident, &x.attrs),
                syn::ImplItem::Const(x) => (&x.ident, &x.attrs),
                _ => None?,
            };

            let id = id.to_string();
            Some(SideItem::new(id, attrs))
        })
    }

    /// Returns struct fields.
    fn struct_fields<'a>(&'a self, item: &'a ItemStruct) -> impl Iterator<Item = SideItem<'a>> {
        return item.fields.iter().enumerate().map(|(i, field)| {
            let id = field_id(i, field);
            let attrs = &field.attrs;
            SideItem::new(id, attrs)
        });

        fn field_id(i: usize, field: &syn::Field) -> String {
            let id = field.ident.as_ref().map(|x| x.to_string());
            id.unwrap_or(tuple_fild_id(i))
        }

        fn tuple_fild_id(i: usize) -> String {
            format!("v{i}")
        }
    }

    /// Returns trait items.
    fn trait_items<'a>(&'a self, item: &'a ItemTrait) -> impl Iterator<Item = SideItem<'a>> {
        item.items.iter().filter_map(|x| {
            let (id, attrs) = match x {
                syn::TraitItem::Fn(x) => (&x.sig.ident, &x.attrs),
                syn::TraitItem::Type(x) => (&x.ident, &x.attrs),
                syn::TraitItem::Const(x) => (&x.ident, &x.attrs),
                _ => None?,
            };

            let id = id.to_string();
            Some(SideItem::new(id, attrs))
        })
    }
}
