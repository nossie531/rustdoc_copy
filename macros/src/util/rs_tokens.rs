//! Provider of [`RsTokens`].

use rustc_lexer::TokenKind;

/// Rust tokens iterator.
pub(crate) struct RsTokens<'a> {
    /// Rust code.
    code: &'a str,
    /// Current cursor position.
    pos: usize,
}

impl<'a> RsTokens<'a> {
    /// Creates a new instance.
    pub fn new(code: &'a str) -> Self {
        Self { code, pos: 0 }
    }
}

impl<'a> Iterator for RsTokens<'a> {
    type Item = RsToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let rest = &self.code[self.pos..];
        if rest.is_empty() {
            None
        } else {
            let token = &rustc_lexer::first_token(rest);
            let code = &self.code[self.pos..(self.pos + token.len)];
            let item = RsToken::new(code, token.kind);
            self.pos += token.len;
            Some(item)
        }
    }
}

/// Rust token.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct RsToken<'a> {
    /// Rust code.
    code: &'a str,
    /// Token kind.
    kind: TokenKind,
}

impl<'a> RsToken<'a> {
    /// Creates a new instance.
    pub fn new(code: &'a str, kind: TokenKind) -> Self {
        Self { code, kind }
    }

    /// Returns Rust code.
    pub fn code(&self) -> &'a str {
        self.code
    }
}
