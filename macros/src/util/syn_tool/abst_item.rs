//! Provider of [`AbstItem`].

use crate::util::syn_tool::*;
use syn::Attribute;

/// Abstracted item.
pub(crate) trait AbstItem {
    /// Returns attributes of item.
    fn attrs(&self) -> &Vec<syn::Attribute>;

    /// Returns side items of item.
    fn sides(&self) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        unimplemented!();
        #[allow(unreachable_code)]
        [].into_iter()
    }
}

impl AbstItem for syn::ItemConst {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }
}

impl AbstItem for syn::ItemFn {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }
}

impl AbstItem for syn::ItemMacro {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }
}

impl AbstItem for syn::ItemMod {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }
}

impl AbstItem for syn::ItemStatic {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }
}

impl AbstItem for syn::ItemType {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }
}

impl AbstItem for syn::ItemEnum {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }

    fn sides(&self) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        self.variants.iter().map(|x| (x.ident.clone(), &x.attrs))
    }
}

impl AbstItem for syn::ItemImpl {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }

    fn sides(&self) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        return self.items.iter().filter_map(side);

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
}

impl AbstItem for syn::ItemStruct {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }

    fn sides(&self) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        return self.fields.iter().enumerate().map(|(i, field)| {
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
}

impl AbstItem for syn::ItemTrait {
    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }

    fn sides(&self) -> impl Iterator<Item = (syn::Ident, &Vec<Attribute>)> {
        return self.items.iter().filter_map(side);

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
