//! Provider of [`print_doc_chunk_mod`].

use crate::doc_parts::*;
use crate::print::{LinkAdjuster, terms::*};
use crate::util::md_tool::md_print::*;
use crate::util::syn_tool::*;
use crate::util::*;
use proc_macro2::TokenStream;
use pulldown_cmark::Event;
use quote::quote;
use std::cell::Ref;
use std::iter;

/// Returns tokens of given [`DocChunkMod`].
pub(crate) fn print_doc_chunk_mod(chunk_mod: &DocChunkMod) -> TokenStream {
    let head_macro = head_macro(chunk_mod);
    let body_macro = body_macro(chunk_mod);
    let defs_macro = defs_macro(chunk_mod);
    let top_macro = top_macro(chunk_mod);
    let subs_macro = subs_macro(chunk_mod);
    let all_macro = all_macro(chunk_mod);
    let sub_mod = sub_mod(chunk_mod);
    let contents = quote! {
        #head_macro
        #body_macro
        #defs_macro
        #top_macro
        #subs_macro
        #all_macro
        #sub_mod
    };

    templates::module(chunk_mod.mod_id(), &contents)
}

/// Returns `head!` macro tokens.
fn head_macro(chunk_mod: &DocChunkMod) -> TokenStream {
    let chunk = &chunk_mod.chunk().borrow();
    let events = adjust_links(chunk, chunk.head_events());
    let md = MdPrinter::print(events);
    let tokens = &syn_tool::text(&md);
    templates::doc_macro(&ids::head(), tokens)
}

/// Returns `body!` macro tokens.
fn body_macro(chunk_mod: &DocChunkMod) -> TokenStream {
    let chunk = &chunk_mod.chunk().borrow();
    let events = adjust_links(chunk, chunk.body_events());
    let md = MdPrinter::print(events);
    let tokens = &syn_tool::text(&md);
    templates::doc_macro(&ids::body(), tokens)
}

/// Returns `defs!` macro tokens.
fn defs_macro(chunk_mod: &DocChunkMod) -> TokenStream {
    if !chunk_mod.is_root() {
        return quote! {};
    }

    let chunk = chunk_mod.chunk().borrow();
    let mds = chunk.defs().map(|(key, url)| format!("[{key}]: {url}"));
    let md = mds.collect::<Vec<_>>().join("\n");
    let tokens = &syn_tool::text(&md);
    templates::doc_macro(&ids::defs(), tokens)
}

/// Returns `subs` macro tokens.
fn subs_macro(chunk_mod: &DocChunkMod) -> TokenStream {
    let path = chunk_mod.mod_path();
    let chunk = chunk_mod.chunk().borrow();
    let chunks = chunk.chunks();
    let sub_ids = chunks.filter_map(|x| x.borrow().rs_id().map(ns::id));
    let sub_macros = sub_ids.map(|x| paths::sub_all_macro(&x));
    let sub_macros_calls = sub_macros.map(|x| quote! { #path::#x!() });
    let concat = &templates::concat(sub_macros_calls);
    templates::doc_macro(&ids::subs(), concat)
}

/// Returns `top!` macro tokens.
fn top_macro(chunk_mod: &DocChunkMod) -> TokenStream {
    let path = chunk_mod.mod_path();
    let all_macros = [paths::head_macro(), paths::body_macro()].into_iter();
    let all_macros_call = all_macros.map(|x| quote! { #path::#x!() });
    let concat = &templates::concat(all_macros_call);
    templates::doc_macro(&ids::top(), concat)
}

/// Returns `all!` macro tokens.
fn all_macro(chunk_mod: &DocChunkMod) -> TokenStream {
    let path = chunk_mod.mod_path();
    let top_macros = iter::once(paths::top_macro());
    let subs_macros = iter::once(paths::subs_macro());
    let defs_macros = chunk_mod.is_root().then_some(paths::defs_macro());
    let all_macros = top_macros.chain(subs_macros).chain(defs_macros);
    let all_macros_calls = all_macros.map(|x| quote! { #path::#x!() });
    let concat = &templates::concat(all_macros_calls);
    templates::doc_macro(&ids::all(), concat)
}

/// Returns `sub` module tokens.
fn sub_mod(chunk_mod: &DocChunkMod) -> TokenStream {
    let path = &ns::path_with(chunk_mod.mod_path(), [&ids::sub()]);
    let chunk = chunk_mod.chunk().borrow();
    let sub_items = chunk.chunks().map(|x| sub_item_mod(path, &x));
    let sub_items_tokens = &quote! { #(#sub_items)* };
    templates::module(&ids::sub(), sub_items_tokens)
}

/// Returns `sub` item module tokens.
fn sub_item_mod(path: &syn::Path, chunk: &DocChunk) -> TokenStream {
    assert!(!chunk.borrow().is_root() && chunk.borrow().rs_id().is_some());
    let id = &ns::id(chunk.borrow().rs_id().unwrap());
    let path = &ns::path_with(path, [id]);
    let chunk_mod = DocChunkMod::new_sub(path, chunk);
    print_doc_chunk_mod(&chunk_mod)
}

/// Adjust links.
fn adjust_links<'a>(
    chunk: &Ref<DocChunkCore<'a>>,
    events: impl Iterator<Item = Event<'a>>,
) -> impl Iterator<Item = Event<'a>> {
    let mut la = LinkAdjuster::new()
        .with_self_item(chunk.self_item().cloned())
        .with_copy_guard(chunk.copy_guard().map(str::to_string));
    events.map(move |x| la.adjust(&x))
}
