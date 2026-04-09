//! Provider of [`FieldNt`].

use crate::util::syn_tools::*;
use std::mem;

/// Newtype of [`syn::Field`].
#[repr(transparent)]
pub(crate) struct FieldNt(syn::Field);

impl SideItem for FieldNt {
    fn id(&self, index: usize) -> String {
        let id = self.0.ident.as_ref().map(|x| x.to_string());
        return id.unwrap_or(for_tuple(index));

        fn for_tuple(i: usize) -> String {
            format!("v{i}")
        }
    }

    fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.0.attrs
    }

    fn set_attrs(&mut self, value: Vec<syn::Attribute>) {
        self.0.attrs = value;
    }
}

impl AsRef<FieldNt> for syn::Field {
    fn as_ref(&self) -> &FieldNt {
        unsafe { mem::transmute(self) }
    }
}

impl AsMut<FieldNt> for syn::Field {
    fn as_mut(&mut self) -> &mut FieldNt {
        unsafe { mem::transmute(self) }
    }
}
