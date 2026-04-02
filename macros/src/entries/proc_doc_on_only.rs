//! Provider of [`proc_doc_on_only`].

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// Process [`doc_on_only`](crate::doc_on_only).
pub(crate) fn proc_doc_on_only(_attr: TokenStream, body: TokenStream) -> TokenStream {
    let mut ret = TokenStream::new();
    let mut doc_flag = false;
    let iter = &mut body.into_iter();

    while let Some((is_doc, tokens)) = parcel_tokens(iter) {
        if !is_doc {
            ret.extend(tokens);
            continue;
        }

        if !doc_flag {
            ret.extend(allow_missing_docs());
            doc_flag = true;
        }

        let tokens = &mut tokens.into_iter();
        let Some(_pound) = tokens.next() else {
            unreachable!()
        };
        let Some(TokenTree::Group(group)) = tokens.next() else {
            unreachable!()
        };
        let doc = cfg_attr(feature("doc_on"), group.stream());
        ret.extend(doc);
    }

    ret
}

/// Parcel tokens from iterator.
fn parcel_tokens(ts: &mut impl Iterator<Item = TokenTree>) -> Option<(bool, TokenStream)> {
    let mut buf = Vec::new();

    buf.extend([ts.next()?]);
    if buf.last().is_none_or(|x| !is_pound(x)) {
        return Some((false, TokenStream::from_iter(buf)));
    }

    buf.extend([ts.next()?]);
    if buf.last().is_none_or(|x| !is_doc_group(x)) {
        return Some((false, TokenStream::from_iter(buf)));
    }

    Some((true, TokenStream::from_iter(buf)))
}

/// Returns `true` if given token is pound mark ('`#`').
fn is_pound(tt: &TokenTree) -> bool {
    let TokenTree::Punct(p) = tt else {
        return false;
    };
    p.as_char() == '#'
}

/// Returns `true` if given token is group started by `"doc"`.
fn is_doc_group(tt: &TokenTree) -> bool {
    let group = match tt {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket => g,
        _ => return false,
    };

    let fst = match group.stream().into_iter().next() {
        Some(TokenTree::Ident(fst)) => fst,
        _ => return false,
    };

    fst.to_string() == "doc"
}

/// Returns token stream of `#[allow(missing_docs)]`.
fn allow_missing_docs() -> TokenStream {
    let missing_docs = TokenStream::from_iter([TokenTree::Ident(Ident::new(
        "missing_docs",
        Span::call_site(),
    ))]);

    let allow_expr = TokenStream::from_iter([
        TokenTree::Ident(Ident::new("allow", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, missing_docs)),
    ]);

    TokenStream::from_iter([
        TokenTree::Punct(Punct::new('#', Spacing::Joint)),
        TokenTree::Group(Group::new(Delimiter::Bracket, allow_expr)),
    ])
}

/// Returns token stream of `#[content]`.
fn attr(content: TokenStream) -> TokenStream {
    TokenStream::from_iter([
        TokenTree::Punct(Punct::new('#', Spacing::Joint)),
        TokenTree::Group(Group::new(Delimiter::Bracket, content)),
    ])
}

/// Returns token stream of `#[cfg_attr(cfg, inside_attr)]`.
fn cfg_attr(cfg: TokenStream, inside_attr: TokenStream) -> TokenStream {
    let cfg_attr_id = TokenTree::Ident(Ident::new("cfg_attr", Span::call_site()));
    let inside_paren = paren([cfg, inside_attr].into_iter());
    attr(TokenStream::from_iter([cfg_attr_id, inside_paren]))
}

/// Returns token stream of `feature = "target"`.
fn feature(target: &str) -> TokenStream {
    TokenStream::from_iter([
        TokenTree::Ident(Ident::new("feature", Span::call_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Literal(Literal::string(target)),
    ])
}

/// Returns token stream of `(arg1, arg2, ...)`.
fn paren(args: impl Iterator<Item = TokenStream>) -> TokenTree {
    let mut contents = TokenStream::new();
    for arg in args {
        if !contents.is_empty() {
            let comma = TokenTree::Punct(Punct::new(',', Spacing::Alone));
            contents.extend([comma]);
        }

        contents.extend(arg);
    }

    TokenTree::Group(Group::new(Delimiter::Parenthesis, contents))
}
