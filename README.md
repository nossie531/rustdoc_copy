# rustdoc_copy

Rustdoc comment copy helper.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## Core items

- [`doc_share`] - Copy rustdoc comment from other rustdoc comment.
- [`doc_file`] - Copy rustdoc comment from Markdown file.

## Highlights

😊 Pros

- Partial copy is supported with various methods.
- Partial copy can work with references and definitions.
- [`doc_file`] supports heading level adjusting.
- [`doc_file`] supports link copy guard (for `docs.rs` URL in `README.md`).

🤔 Cons

- [No root document sharing][c1] (expecting futrue Rust).
- [No file update detecting][c2] (expecting futrue Rust).
- [Miss copy of document][c3] (expecting futrue crates for Markdown).

[c1]: #no-root-document-shareing
[c2]: #no-file-update-detecting
[c3]: #miss-copy-of-document

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

Document components is specified by path with following components.

1. Root module

   From first argument of [`doc_share`] and [`doc_file`].

2. Item module

   | ID                                   | Description        |
   | --                                   | --                 |
   | <em>No selection</em>                | For no member item |
   | <code>base</code>                    | Base item          |
   | <code>side::<var>member</var></code> | Member item        |

3. Section module (recursive)

   | ID                                   | Description   |
   | --                                   | --            |
   | <em>No selection</em>                | Root section  |
   | <code>sub::<var>section</var></code> | Named section |

4. Parts macro

   | ID      | Description                                   |
   | --      | --                                            |
   | `head!` | First block (Mrakdown heading fit this group) |
   | `body!` | Subsequent blocks                             |
   | `defs!` | Definitions (Root only)                       |
   | `subs!` | All sub sections                              |
   | `top!`  | `head!` + `body!`                             |
   | `all!`  | `top!` + `subs!` + `defs!`                    |

### ID from title

Section identification by title has two subtly different styles.

- _Common rule_

  - All uppercase characters are converted to lowercase.
  - Special characters like puctuation and Emoji are ignored.

- Global style

  - This style is used in fragment key in [`doc_file`].
  - This style follows heading anchor rule of GFM.
  - Spaces in the titles are replaced into hyphen ('`-`').
  - Sequential numers are used if same titles exist in the one document.

- Local style
  
  - This style is used in section ID in [document path](#document-path).
  - This style follows naming rule of Rust module.
  - Spaces in the titles are replaced into underscore ('`_`').
  - Sequential numbers are used if same title exists in sibling sections.
  - Long ID or path can shorten with `use` and `as` keywords.

### References and definitions

Up for partial copy, all links and images definitions are embeded to references.

About footnotes, use `defs!` macro. Because they lack embeding ability.

### No root document shareing

At `doc_share`, root documents of Rust files are not supported. This is because
notation like `#![doc_share(doc)]` are not supported. (I wonder this issue will
be resolved by [inner macro attribute][rust_issues_54726] in the future.)

[rust_issues_54726]: https://github.com/rust-lang/rust/issues/54726

### No file update detecting

At `doc_file`, argument file is not tracked on real time from IDE. Therefore,
error detection and autocompletion at IDE is based on old Markdown headings.
This state will remain until rebuild. (I wonder this issue will be resolved
by [`proc_macro::tracked::path`][ptp] in the future.)

[ptp]: https://doc.rust-lang.org/proc_macro/tracked/fn.path.html

### Miss copy of document

The content of documents may change slightly when copied. This is the impact
of the crate for CommonMark that this crate depends on. The crate is high
precision, but currently, not perfect. Therefore, when using elaborate
notation, please inspect the outputs by yourself.

### Common compile error

The path of root module for documentation must be accessible without
preceding paths. In other words, if that identifier is in another module,
`use` keyword is required.

-&nbsp;Compilation successed

```rust
pub mod my_mod {
    use rustdoc_copy::prelude::*;

    /// This is my function.
    #[doc_share(doc)]
    pub fn my_func() {
        println!("`my_func` is called.");
    }
}

use rustdoc_copy::prelude::*;
use my_mod::doc;

#[doc = doc::all!()]
pub fn my_func_alias() {
    my_mod::my_func();
}
```

-&nbsp;Compilation failed!

```rust, compile_fail
pub mod my_mod {
    use rustdoc_copy::prelude::*;

    /// This is my function.
    #[doc_share(doc)] // <---- ❌ Error detected.
    pub fn my_func() {
        println!("`my_func` is called.");
    }
}

use rustdoc_copy::prelude::*;
// use my_mod::doc; // <---- 🚧 Shorthand path is not used.

#[doc = my_mod::doc::all!()] // <---- 👈 Full path is used.
pub fn my_func_alias() {
    my_mod::my_func();
}
```

## History

See [CHANGELOG](CHANGELOG.md).

<!-- links -->

[!copy_guard]: https://docs.rs/rustdoc_copy/0.1.0/
[`doc_share`]: https://docs.rs/rustdoc_copy/0.1.0/rustdoc_copy/attr.doc_share.html
[`doc_file`]: https://docs.rs/rustdoc_copy/0.1.0/rustdoc_copy/macro.doc_file.html
