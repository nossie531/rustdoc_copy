//! Provider of [`DocShare`].

use crate::doc::*;
use crate::util::syn_tool::*;
use crate::util::*;
use crate::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Error;
use syn::parse::{Parse, ParseStream};

/// Syntax node of `doc_share` attribute macro.
pub(crate) struct DocShare {
    /// ID of document module.
    mod_id: Option<syn::Ident>,
    /// Target item.
    syn_item: syn::Item,
}

impl DocShare {
    /// Translate input tokens to output tokens.
    pub fn translate(attr: TokenStream, body: TokenStream) -> TokenStream {
        match Self::parse_all(attr, body) {
            Err(x) => x,
            Ok(x) => x.into_token_stream(),
        }
    }

    /// Parse all inputs.
    pub fn parse_all(attr: TokenStream, body: TokenStream) -> Result<Self, TokenStream> {
        // Prase attribute and body.
        let mod_id = syn_tool::parse::<syn::Ident>(attr);
        let body = syn_tool::parse::<Self>(body);

        // Process errors.
        let valid = body.as_ref().map_or_else(|_| Ok(()), Self::validate);
        let errs = []
            .iter()
            .chain(mod_id.as_ref().err())
            .chain(body.as_ref().err())
            .chain(valid.as_ref().err())
            .collect::<Vec<_>>();
        if !errs.is_empty() {
            return Err(quote::quote! { #(#errs)* });
        }

        // Return result.
        let mut ret = body.unwrap();
        ret.mod_id = Some(mod_id.unwrap());
        Ok(ret)
    }

    /// Returns ID.
    pub fn mod_id(&self) -> &syn::Ident {
        self.mod_id.as_ref().unwrap()
    }

    /// Returns item.
    pub fn syn_item(&self) -> &syn::Item {
        &self.syn_item
    }

    /// Validate inputs.
    pub fn validate(&self) -> Result<(), TokenStream> {
        if !(self.syn_item.as_ref() as &BaseItem).is_documentable() {
            let item = SkipAttr::reparse(&self.syn_item);
            let err = Error::new_spanned(item, msg::UNEXPECTED_ITEM);
            let err = err.to_compile_error();
            return Err(err);
        }

        Ok(())
    }
}

impl Parse for DocShare {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let syn_item = input.parse::<syn::Item>()?;
        Ok(Self {
            mod_id: None,
            syn_item,
        })
    }
}

impl ToTokens for DocShare {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mod_id = self.mod_id();
        let syn_item = &self.syn_item();
        let doc_item = &DocShareMod::new(mod_id, syn_item);
        tokens.extend(syn_item.into_token_stream());
        tokens.extend(print::print_doc_share_mod(doc_item));
    }
}
