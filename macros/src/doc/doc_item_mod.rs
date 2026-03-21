use crate::util::syn_tool::*;

pub(crate) struct DocItemMod<'a> {
    mod_id: &'a syn::Ident,
    item: &'a BaseItem,
}

impl<'a> DocItemMod<'a> {
    pub fn new(mod_id: &'a syn::Ident, item: &'a syn::Item) -> Self {
        let item = BaseItem::from_ref(item);
        Self { mod_id, item }
    }

    pub fn mod_id(&self) -> &'a syn::Ident {
        self.mod_id
    }

    pub fn item(&self) -> &'a BaseItem {
        self.item
    }
}
