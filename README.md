# rustdoc_copy

Rustdoc comment copy helper.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## Core items

- [`doc_share`] - Copy rustdoc comment from other rustdoc comment.
- [`doc_file`] - Copy rustdoc comment from Markdown file.
- [`doc_on_only`] - Stop documentation for build speed.

## Highlights

😊 Pros

- [Document path][p1] supports partial copy in various methods.
- [Link adjustments][p2] prevents broken links in various situations.
- [`doc_file`] supports heading level adjusting.

[p1]: #document-path
[p2]: #link-adjustments

🤔 Cons

- [No tooltip display at IDE][c1] (expecting future IDE).
- [No file update detecting at IDE][c2] (expecting futrue Rust).
- [No root document sharing][c3] (expecting futrue Rust).
- [Miss copy of document][c4] (expecting futrue crates for Markdown).
- [Mysterious compilation troubles][c5] (maybe not a big deal).

[c1]: #no-tooltip-display-at-ide
[c2]: #no-file-update-detecting-at-ide
[c3]: #no-root-document-shareing
[c4]: #miss-copy-of-document
[c5]: #mysterious-compilation-troubles

## Examples

### Simple copy

```rust
use rustdoc_copy::prelude::*;

/// This is my function.
#[doc_share(doc)]
pub fn my_func() {
    println!("`my_func` is called.");
}

#[doc = doc::all!()]
pub fn my_func_alias() {
    my_func();
}
```

### Partial copy

```rust
use rustdoc_copy::prelude::*;

/// This is my function.
/// 
/// # Some Notes
/// 
/// Message print only.
#[doc_share(doc)]
pub fn my_func() {
    println!("`my_func` is called.");
}

/// This is my function alias.
#[doc = doc::sub::some_notes::all!()]
pub fn my_func_alias() {
    my_func();
}
```

### With members

```rust
use rustdoc_copy::prelude::*;

/// This is my struct.
#[doc_share(doc)]
pub struct MyStruct {
    /// Field 1.
    pub field1: i32,
    /// Field 2.
    pub field2: i32,
}

#[doc_share(doc_impl)]
impl MyStruct {
    /// Method 1.
    fn method1(&self) {}
    /// Method 2.
    fn method2(&self) {}
}

#[doc = doc::base::all!()]
pub struct MyStructAlias {
    #[doc = doc::side::field1::all!()]
    pub field1: i32,
    #[doc = doc::side::field2::all!()]
    pub field2: i32,
}

impl MyStructAlias {
    #[doc = doc_impl::side::method1::all!()]
    fn method1(&self) {}
    #[doc = doc_impl::side::method2::all!()]
    fn method2(&self) {}
}
```

### File copy

-&nbsp;`src/lib.rs`

```rust, ignore
//! Welcome to my crate API document.
#![doc = doc::sub::examples::all!()]

doc_file!(doc, "README.md#");

use rustdoc_copy::prelude::*;

pub fn some_func() -> i32 {
    42
}
```

-&nbsp;`README.md`

````text
# my_crate

This crate is ...

## Examples

```rust
use my_crate::some_func;

assert_eq!(some_func(), 42);
```
````

## Tips

### Document path

Partial document is specified by path with following components.

1. **Root module**

   Select root by first argument of [`doc_share`] and [`doc_file`].

2. **Fragment key**

   Select fragment key after file path ([`doc_file`] only).

   See "[Fragment key][doc_file#fragment-key]" section in [`doc_file`]
   for more details.

3. **Item module**

   Select Rust item in target root or fragment.

   | ID                                   | Description        |
   | --                                   | --                 |
   | <em>No selection</em>                | For no member item |
   | <code>base</code>                    | Base item          |
   | <code>side::<var>member</var></code> | Member item        |

4. **Section module**

   Select sub section _recursively_ in target Rust item.

   | ID                                   | Description   |
   | --                                   | --            |
   | <em>No selection</em>                | Root section  |
   | <code>sub::<var>section</var></code> | Named section |

5. **Parts macro**

   Select block parts in target section.

   | ID      | Description                                   |
   | --      | --                                            |
   | `head!` | First block (Mrakdown heading fit this group) |
   | `body!` | Subsequent blocks                             |
   | `defs!` | Definitions (Root only)                       |
   | `subs!` | All sub sections                              |
   | `top!`  | `head!` + `body!`                             |
   | `all!`  | `top!` + `subs!` + `defs!`                    |

### Title ID

Section identification by title has two subtly different styles.

- _**Common rule**_

  - All uppercase characters are converted to lowercase.
  - Special characters like punctuation and Emoji are ignored.

- **Global style**

  - This style is used in fragment key in [`doc_file`].
  - This style follows heading anchor rule of GFM.
  - Spaces in the titles are replaced into hyphen ('`-`').
  - Sequential numers are used if same titles exist in the one document.

- **Local style**
  
  - This style is used in section ID in [document path](#document-path).
  - This style follows naming rule of Rust module.
  - Spaces in the titles are replaced into underscore ('`_`').
  - Sequential numbers are used if same title exists in sibling sections.
  - Long ID or path can shorten with `use` and `as` keywords.

### Link adjustments

To prevent broken links, folowing mechanisms are supported.

- **Link embeding**

  Up for partial copy, reference style link is converted to inline style.

  (In contrast, footnotes are not adjusted. Because they lack embeding ability.
  So, use `defs!` macro instead.)

- **Special `Self`**

  Up for `Self` keyword meaning changes, `doc_share::Self` is prepared.

  See ["`doc_share::Self`"][doc_share#doc-share-self] section in [`doc_share`]
  document for more detail.

  (In contrast, `self` and `super` keywords do not have a mechanism for
  `doc_share`. Because attribute macros can not know module of item.
  So, use target name directly instead.)

- **Copy guard**

  Up for API links in `README.md`, URLs under the copy guard path are ignored.

  See ["Link copy guard"][doc_file#link-copy-guard] section in [`doc_file`]
  document for more detail.

## Trouble shooting

### No tooltip display at IDE

This crate uses the `doc` attribute frequently for document importing.
But documentation by `doc` attributes are reflected only rustdoc file,
not IDE tooltips (At least VS Code with rust-analyzer in 2026).

### No file update detecting at IDE

At `doc_file`, argument file is not tracked on real time from IDE. Therefore,
error detection and autocompletion at IDE is based on old Markdown headings.
This state will remain until rebuild. (I wonder this issue will be resolved
by [`proc_macro::tracked::path`][ptp] in the future.)

[ptp]: https://doc.rust-lang.org/proc_macro/tracked/fn.path.html

### No root document shareing

At `doc_share`, root documents of Rust files are not supported. This is because
notation like `#![doc_share(doc)]` are not supported. (I wonder this issue will
be resolved by [inner macro attribute][rust_issues_54726] in the future.)

[rust_issues_54726]: https://github.com/rust-lang/rust/issues/54726

### Miss copy of document

The content of documents may change slightly when copied. This is the impact
of the crate for CommonMark that this crate depends on. The crate is high
precision, but currently, not perfect. Therefore, when using elaborate
notation, please inspect the outputs by yourself.

### Mysterious compilation troubles

Following compilation troubles maybe a bit tough.

- [Normal path error][docs::normal_path_error]
- [Self path warning][docs::self_path_warning]

## History

See [CHANGELOG](CHANGELOG.md).

<!-- links -->

[!copy_guard]: https://docs.rs/rustdoc_copy/0.2.0/
[`doc_share`]: https://docs.rs/rustdoc_copy/0.2.0/rustdoc_copy/attr.doc_share.html
[`doc_file`]: https://docs.rs/rustdoc_copy/0.2.0/rustdoc_copy/macro.doc_file.html
[`doc_on_only`]: https://docs.rs/rustdoc_copy/0.2.0/rustdoc_copy/attr.doc_on_only.html
[docs::normal_path_error]: https://docs.rs/rustdoc_copy/0.2.0/rustdoc_copy/docs/normal_path_error/index.html
[docs::self_path_warning]: https://docs.rs/rustdoc_copy/0.2.0/rustdoc_copy/docs/self_path_error/index.html
[doc_file#fragment-key]: https://docs.rs/rustdoc_copy/0.2.0/rustdoc_copy/macro.doc_file.html#fragment-key
[doc_file#link-copy-guard]: https://docs.rs/rustdoc_copy/0.2.0/rustdoc_copy/macro.doc_file.html#link-copy-guard
[doc_share#doc-share-self]: https://docs.rs/rustdoc_copy/0.2.0/rustdoc_copy/attr.doc_share.html#doc-share-self