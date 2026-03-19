//! Naming rules.

use std::ops::Range;

/// Returns fragment ID from the given text.
pub(crate) fn to_fragment_id(text: &str) -> String {
    let mut ret = String::new();
    for c in trim_none_xid(text).chars() {
        if c == ' ' {
            ret.push('-')
        } else if unicode_ident::is_xid_continue(c) {
            ret.extend(c.to_lowercase());
        }
    }

    ret
}

/// Returns LSC (Lower Snake Case) Rust ID from the given text.
pub(crate) fn to_lsc_rust_id(text: &str) -> String {
    let mut ret = String::new();

    for (i, c) in trim_none_xid(text).chars().enumerate() {
        if i == 0 && !unicode_ident::is_xid_start(c) {
            ret.push('_');
        }

        if c == ' ' {
            ret.push('_')
        } else if unicode_ident::is_xid_continue(c) {
            ret.extend(c.to_lowercase());
        }
    }

    ret
}

/// Returns trimed text.
fn trim_none_xid(text: &str) -> &str {
    let mut range = None as Option<Range<usize>>;
    for (i, ch) in text.char_indices() {
        if unicode_ident::is_xid_continue(ch) {
            range = range.or(Some(i..(i + 1)));
            range.as_mut().unwrap().end = i + 1;
        }
    }

    match range {
        None => &text[0..0],
        Some(x) => &text[x],
    }
}

#[test]
fn test_to_lower_snake_case_rust_id() {
    let datas = [
        ("", "".to_string()),
        (" ", "".to_string()),
        ("🚩", "".to_string()),
        ("🚩 ", "".to_string()),
        ("word", "word".to_string()),
        (" word ", "word".to_string()),
        ("🚩word🚩", "word".to_string()),
        ("🚩 word 🚩", "word".to_string()),
        ("9word", "_9word".to_string()),
        ("word9", "word9".to_string()),
        ("SomeWord", "someword".to_string()),
    ];

    for (text, tobe) in datas {
        let asis = to_lsc_rust_id(text);
        assert_eq!(asis, tobe);
    }
}
