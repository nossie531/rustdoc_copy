//! Provider of [`VariantNt`].

use crate::util::syn_tools::*;
use std::mem;

/// Newtype of [`syn::Variant`].
#[repr(transparent)]
pub(crate) struct VariantNt(syn::Variant);

impl SideItem for VariantNt {
    fn id(&self, _index: usize) -> String {
        self.0.ident.to_string()
    }

    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.0.attrs
    }

    fn set_attrs(&mut self, value: Vec<syn::Attribute>) {
        self.0.attrs = value;
    }
}

impl AsRef<VariantNt> for syn::Variant {
    fn as_ref(&self) -> &VariantNt {
        unsafe { mem::transmute(self) }
    }
}

impl AsMut<VariantNt> for syn::Variant {
    fn as_mut(&mut self) -> &mut VariantNt {
        unsafe { mem::transmute(self) }
    }
}
