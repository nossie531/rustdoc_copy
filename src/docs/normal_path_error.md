<!-- About normal path error -->

The path of root module for documentation
must be accessible without preceding paths.

In other words, if that identifier is in
another module, `use` keyword is required.

# Examples

Compilation of following code will success.

```rust
pub mod my_mod {
    use rustdoc_copy::prelude::*;

    /// This is my function.
    #[doc_share(doc)]
    pub fn my_func() {
        println!("`my_func` is called.");
    }
}

use my_mod::doc;

#[doc = doc::all!()]
pub fn my_func_alias() {
    my_mod::my_func();
}
```

Compilation of following code cause error.

```rust, compile_fail
pub mod my_mod {
    use rustdoc_copy::prelude::*;

    /// This is my function.
    #[doc_share(doc)] // <---- ❌ Error detected.
    pub fn my_func() {
        println!("`my_func` is called.");
    }
}

// use my_mod::doc; // <---- 🚧 Shorthand path is not used.

#[doc = my_mod::doc::all!()] // <---- 👈 Full path is used.
pub fn my_func_alias() {
    my_mod::my_func();
}
```
