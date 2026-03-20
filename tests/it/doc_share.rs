use rustdoc_copy::prelude::*;

#[test]
fn test_const() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Target document.");

    /// Target document.
    #[doc_share(doc)]
    #[allow(unused)]
    const TARGET_ITEM: i32 = 42;
}

#[test]
fn test_fn() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Target document.");

    /// Target document.
    #[doc_share(doc)]
    #[allow(unused)]
    fn target_item() {}
}

#[test]
fn test_macro() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Target document.");

    /// Target document.
    #[doc_share(doc)]
    #[allow(unused)]
    macro_rules! target_item {
        () => {};
    }
}

#[test]
fn test_mod() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Target document.");

    /// Target document.
    #[doc_share(doc)]
    #[allow(unused)]
    mod target_item {}
}

#[test]
fn test_static() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Target document.");

    /// Target document.
    #[doc_share(doc)]
    #[allow(unused)]
    static TARGET_ITEM: i32 = 42;
}

#[test]
fn test_type() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Target document.");

    /// Target document.
    #[doc_share(doc)]
    #[allow(unused)]
    type TargetItem = i32;
}

#[test]
fn test_enum() {
    let datas = [
        (doc::base::all!(), "Base document."),
        (doc::side::Variant1::all!(), "Variant 1."),
        (doc::side::Variant2::all!(), "Variant 2."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    /// Base document.
    #[doc_share(doc)]
    #[allow(unused)]
    enum TargetItem {
        /// Variant 1.
        Variant1,
        /// Variant 2.
        Variant2,
    }
}

#[test]
fn test_impl() {
    let datas = [
        (doc::base::all!(), "Base document."),
        (doc::side::CONST::all!(), "Some const."),
        (doc::side::method::all!(), "Some method."),
        (doc_impl_for::side::Type::all!(), "Some type."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    #[allow(unused)]
    struct SomeStruct();

    #[allow(unused)]
    trait SomeTrait {
        type Type;
    }

    /// Base document.
    #[doc_share(doc)]
    #[allow(unused)]
    impl SomeStruct {
        /// Some const.
        const CONST: i32 = 42;
        /// Some method.
        fn method() {}
    }

    #[doc_share(doc_impl_for)]
    impl SomeTrait for SomeStruct {
        /// Some type.
        type Type = i32;
    }
}

#[test]
fn test_struct_normal() {
    let datas = [
        (doc::base::all!(), "Base document."),
        (doc::side::field1::all!(), "Field 1."),
        (doc::side::field2::all!(), "Field 2."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    /// Base document.
    #[doc_share(doc)]
    #[allow(unused)]
    struct TargetItem {
        /// Field 1.
        pub field1: i32,
        /// Field 2.
        pub field2: i32,
    }
}

#[test]
fn test_struct_tuple() {
    let datas = [
        (doc::base::all!(), "Base document."),
        (doc::side::v0::all!(), "First value."),
        (doc::side::v1::all!(), "Second value."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    /// Base document.
    #[doc_share(doc)]
    #[allow(unused)]
    struct TargetItem(
        /// First value.
        pub i32,
        /// Second value.
        pub i32,
    );
}

#[test]
fn test_trait() {
    let datas = [
        (doc::base::all!(), "Base document."),
        (doc::side::CONST::all!(), "Some const."),
        (doc::side::Type::all!(), "Some type."),
        (doc::side::method::all!(), "Some method."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    /// Base document.
    #[doc_share(doc)]
    #[allow(unused)]
    trait TargetItem {
        /// Some const.
        const CONST: i32;
        /// Some type.
        type Type;
        /// Some method.
        fn method();
    }
}
