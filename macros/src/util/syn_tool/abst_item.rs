//! Provider of [`AbstItem`].

use crate::util::syn_tool::*;
use syn::{Attribute, ItemEnum, ItemImpl, ItemStruct, ItemTrait};

/// Abstracted item.
#[repr(transparent)]
pub(crate) struct AbstItem(syn::Item);

impl AbstItem {
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
    pub fn sides(&self) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        let ret: Box<dyn Iterator<Item = _>> = match &self.0 {
            syn::Item::Enum(x) => Box::new(Self::enum_variants(x)),
            syn::Item::Impl(x) => Box::new(Self::impl_items(x)),
            syn::Item::Struct(x) => Box::new(Self::struct_fields(x)),
            syn::Item::Trait(x) => Box::new(Self::trait_items(x)),
            _ => Box::new(vec![].into_iter()),
        };
        
        ret
    }

    /// Returns variants of `enum`.
    fn enum_variants(item: &ItemEnum) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        item.variants.iter().map(|x| (x.ident.clone(), &x.attrs))
    }

    /// Returns implement items.
    fn impl_items(item: &ItemImpl) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        return item.items.iter().filter_map(side);

        fn side(impl_item: &syn::ImplItem) -> Option<(syn::Ident, &Vec<Attribute>)> {
            let (id, attrs) = match impl_item {
                syn::ImplItem::Fn(x) => (&x.sig.ident, &x.attrs),
                syn::ImplItem::Type(x) => (&x.ident, &x.attrs),
                syn::ImplItem::Const(x) => (&x.ident, &x.attrs),
                _ => None?,
            };

            Some((id.clone(), attrs))
        }
    }

    /// Returns struct fields.
    fn struct_fields(item: &ItemStruct) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        return item.fields.iter().enumerate().map(|(i, field)| {
            let id = field_id(i, field);
            let attrs = &field.attrs;
            (id, attrs)
        });

        fn field_id(i: usize, field: &syn::Field) -> syn::Ident {
            let id = field.ident.as_ref().map(|x| x.to_string());
            ns::id(&id.unwrap_or(tuple_fild_id(i)))
        }

        fn tuple_fild_id(i: usize) -> String {
            format!("v{i}")
        }
    }

    /// Returns trait items.
    fn trait_items(item: &ItemTrait) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        return item.items.iter().filter_map(side);

        fn side(impl_item: &syn::TraitItem) -> Option<(syn::Ident, &Vec<Attribute>)> {
            let (id, attrs) = match impl_item {
                syn::TraitItem::Fn(x) => (&x.sig.ident, &x.attrs),
                syn::TraitItem::Type(x) => (&x.ident, &x.attrs),
                syn::TraitItem::Const(x) => (&x.ident, &x.attrs),
                _ => None?,
            };

            Some((id.clone(), attrs))
        }
    }
}
