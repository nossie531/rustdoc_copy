//! Provider of [`DocFileNode`].

use crate::doc_parts::*;
use crate::util::md_tool::*;
use crate::util::*;
use crate::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::fs;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Token};

/// Syntax node of [`doc_file`] macro.
pub(crate) struct DocFileNode {
    /// ID of document module.
    mod_id: syn::Ident,
    /// Markdown document path.
    md_path: syn::LitStr,
}

impl DocFileNode {
    /// Translate input tokens to output tokens.
    pub fn translate(input: TokenStream) -> TokenStream {
        match syn_tool::parse::<Self>(input) {
            Ok(x) => x.into_token_stream(),
            Err(x) => x.into_token_stream(),
        }
    }
}

impl Parse for DocFileNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mod_id = input.parse()?;
        let _sep = input.parse::<Token![,]>()?;
        let md_path = input.parse::<syn::LitStr>()?;
        Ok(Self { mod_id, md_path })
    }
}

impl ToTokens for DocFileNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Parse Markdown path.
        let md_path = self.md_path.value();
        let md_path = MdPath::from(md_path.as_str());

        // Read Markdown from file path.
        let md = &match fs::read_to_string(md_path.path()) {
            Ok(x) => x,
            Err(x) => {
                let err_pos = self.md_path.to_token_stream();
                let err_msg = Error::new_spanned(err_pos, x);
                tokens.extend(err_msg.into_compile_error());
                return;
            }
        };

        // Extract document chunk from fragment key.
        let chunk = &mut parse::parse_md_file(md);
        let chunk = &match chunk.extract(md_path.key()) {
            Some(x) => x,
            None => {
                let err_pos = self.md_path.to_token_stream();
                let err_msg = Error::new_spanned(err_pos, msg::FRAGMENT_NOT_FOUND);
                tokens.extend(err_msg.into_compile_error());
                return;
            }
        };

        // Wrap document chunk as document module.
        let id = &self.mod_id;
        let doc_mod = &DocFileMod::new(id, chunk);

        // Convert document chunk to tokens.
        tokens.extend(print::print_doc_file_mod(doc_mod));
    }
}
