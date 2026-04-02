<!-- About self path error -->

`Self` is converted actual item ID, but not actual path.

So, the document with `Self` sometimes requires using `use` keyword.

# Examples

Documentation of following code generate _no_ warning.

```rust
pub mod my_mod {
    use rustdoc_copy::prelude::*;

    /// This enum has [`Self::Foo`].
    #[doc_share(doc)]
    pub enum MyEnum {
        Foo
    }
}

use my_mod::doc;
use my_mod::MyEnum;

#[doc = doc::base::all!()]
pub type MyEnumAlias = MyEnum;
```

Documentation of following code generate warning.

```rust
pub mod my_mod {
    use rustdoc_copy::prelude::*;

    /// This enum has [`Self::Foo`]. // <---- ⚠️ Warning reason.
    #[doc_share(doc)]
    pub enum MyEnum {
        Foo
    }
}

use my_mod::doc;
// use my_mod::MyEnum; // <---- 🚧 Shorthand path is not used.

#[doc = doc::base::all!()]
pub type MyEnumAlias = my_mod::MyEnum; // <---- 👈 Full path is used.
```
