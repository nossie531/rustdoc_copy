//! Tools for printing document chunk.

use crate::doc::terms::*;
use crate::doc::*;
use crate::util::md_tool::md_print::*;
use crate::util::syn_tool::*;
use crate::util::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter;

/// Returns document chunk tokens.
pub(crate) fn print_doc(chunk: &DocChunk, path: &syn::Path) -> TokenStream {
    doc_node_print(&ChunkForPrint::new(chunk, path))
}

/// Returns document node tokens.
fn doc_node_print(chunk: &ChunkForPrint) -> TokenStream {
    let head_macro = head_macro(chunk);
    let body_macro = body_macro(chunk);
    let defs_macro = defs_macro(chunk);
    let top_macro = top_macro(chunk);
    let subs_macro = subs_macro(chunk);
    let all_macro = all_macro(chunk);
    let sub_mod = sub_mod(chunk);

    quote! {
        #head_macro
        #body_macro
        #defs_macro
        #top_macro
        #subs_macro
        #all_macro
        #sub_mod
    }
}

/// Returns `head!` macro tokens.
fn head_macro(chunk: &ChunkForPrint) -> TokenStream {
    let chunk = chunk.base.borrow();
    let md = MdPrinter::print(chunk.head_events());
    let tokens = &syn_tool::text(&md);
    templates::doc_macro(&ids::head(), tokens)
}

/// Returns `body!` macro tokens.
fn body_macro(chunk: &ChunkForPrint) -> TokenStream {
    let chunk = chunk.base.borrow();
    let md = MdPrinter::print(chunk.body_events());
    let tokens = &syn_tool::text(&md);
    templates::doc_macro(&ids::body(), tokens)
}

/// Returns `defs!` macro tokens.
fn defs_macro(chunk: &ChunkForPrint) -> TokenStream {
    if !chunk.defs {
        return quote! {};
    }

    let chunk = chunk.base.borrow();
    let mds = chunk.defs().map(|(key, url)| format!("[{key}]: {url}"));
    let md = mds.collect::<Vec<_>>().join("\n");
    let tokens = &syn_tool::text(&md);
    templates::doc_macro(&ids::defs(), tokens)
}

/// Returns `subs` macro tokens.
fn subs_macro(chunk: &ChunkForPrint) -> TokenStream {
    let path = chunk.path;
    let chunk = chunk.base.borrow();
    let chunks = chunk.chunks();
    let sub_ids = chunks.filter_map(|x| x.borrow().rust_id().map(ns::id));
    let sub_macros = sub_ids.map(paths::sub_all_macro);
    let sub_macros_calls = sub_macros.map(|x| quote! { #path::#x!() });
    let concat = &templates::concat(sub_macros_calls);
    templates::doc_macro(&ids::subs(), concat)
}

/// Returns `top!` macro tokens.
fn top_macro(chunk: &ChunkForPrint) -> TokenStream {
    let path = chunk.path;
    let all_macros = [paths::head_macro(), paths::body_macro()].into_iter();
    let all_macros_call = all_macros.map(|x| quote! { #path::#x!() });
    let concat = &templates::concat(all_macros_call);
    templates::doc_macro(&ids::top(), concat)
}

/// Returns `all!` macro tokens.
fn all_macro(chunk: &ChunkForPrint) -> TokenStream {
    let path = chunk.path;
    let top_macros = iter::once(paths::top_macro());
    let subs_macros = iter::once(paths::subs_macro());
    let defs_macros = chunk.defs.then_some(paths::defs_macro());
    let all_macros = top_macros.chain(subs_macros).chain(defs_macros);
    let all_macros_calls = all_macros.map(|x| quote! { #path::#x!() });
    let concat = &templates::concat(all_macros_calls);
    templates::doc_macro(&ids::all(), concat)
}

/// Returns `sub` module tokens.
fn sub_mod(chunk: &ChunkForPrint) -> TokenStream {
    let new_path = &ns::path_with(chunk.path, [&ids::sub()]);
    let chunk = chunk.base.borrow();
    let mod_items = chunk.chunks().map(|x| sub_item_mod(&x, new_path));
    let mod_items_tokens = &quote! { #(#mod_items)* };
    templates::module(&ids::sub(), mod_items_tokens)
}

/// Returns `sub` item module tokens.
fn sub_item_mod(chunk: &DocChunk, path: &syn::Path) -> TokenStream {
    assert!(!chunk.borrow().is_root() && chunk.borrow().rust_id().is_some());
    let mod_id = &ns::id(chunk.borrow().rust_id().unwrap());
    let mod_path = &ns::path_with(path, [mod_id]);
    let node = ChunkForPrint::new(chunk, mod_path).with_defs(false);
    let mod_content_tokens = &doc_node_print(&node);
    templates::module(mod_id, mod_content_tokens)
}
