use rustdoc_copy::prelude::*;
use std::marker::PhantomData;

#[test]
fn test_const() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Document of [`Self`](TARGET_ITEM).");

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    const TARGET_ITEM: i32 = 42;
}

#[test]
fn test_fn() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Document of [`Self`](target_item).");

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    fn target_item() {}
}

#[test]
fn test_macro() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Document of [`Self`](target_item).");

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    macro_rules! target_item {
        () => {};
    }
}

#[test]
fn test_mod() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Document of [`Self`](target_item).");

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    mod target_item {}
}

#[test]
fn test_static() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Document of [`Self`](TARGET_ITEM).");

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    static TARGET_ITEM: i32 = 42;
}

#[test]
fn test_type() {
    let asis = doc::all!();
    assert_eq!(asis.trim(), "Document of [`Self`](TargetItem).");

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    type TargetItem = i32;
}

#[test]
fn test_enum() {
    #[rustfmt::skip]
    let datas = [
        (doc::base::all!(), "Document of [`Self`](TargetItem)."),
        (doc::side::Variant1::all!(), "1st item of [`Self`](TargetItem)."),
        (doc::side::Variant2::all!(), "2nd item of [`Self`](TargetItem)."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    enum TargetItem {
        /// 1st item of [`Self`].
        Variant1,
        /// 2nd item of [`Self`].
        Variant2,
    }
}

#[test]
fn test_impl_inherent() {
    #[rustfmt::skip]
    let datas = [
        (doc::base::all!(), "Document of [`Self`](SomeType)."),
        (doc::side::CONST::all!(), "1st item of [`Self`](SomeType)."),
        (doc::side::method::all!(), "2nd item of [`Self`](SomeType)."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    #[allow(unused)]
    struct SomeType();

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    impl SomeType {
        /// 1st item of [`Self`].
        const CONST: i32 = 42;
        /// 2nd item of [`Self`].
        fn method() {}
    }
}

#[test]
fn test_impl_trait() {
    #[rustfmt::skip]
    let datas = [
        (doc::base::all!(), "Document of [`Self`](SomeType)."),
        (doc::side::Type::all!(), "1st item of [`Self`](SomeType)."),
        (doc::side::method::all!(), "2nd item of [`Self`](SomeType)."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    #[allow(unused)]
    struct SomeType();

    #[allow(unused)]
    trait SomeTrait {
        type Type;
        fn method();
    }

    /// Document of [`Self`].
    #[doc_share(doc)]
    impl SomeTrait for SomeType {
        /// 1st item of [`Self`].
        type Type = i32;
        /// 2nd item of [`Self`].
        fn method() {}
    }
}

#[test]
fn test_struct_normal() {
    #[rustfmt::skip]
    let datas = [
        (doc::base::all!(), "Document of [`Self`](TargetItem)."),
        (doc::side::field1::all!(), "1st item of [`Self`](TargetItem)."),
        (doc::side::field2::all!(), "2nd item of [`Self`](TargetItem)."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    struct TargetItem {
        /// 1st item of [`Self`].
        pub field1: i32,
        /// 2nd item of [`Self`].
        pub field2: i32,
    }
}

#[test]
fn test_struct_tuple() {
    #[rustfmt::skip]
    let datas = [
        (doc::base::all!(), "Document of [`Self`](TargetItem)."),
        (doc::side::v0::all!(), "1st item of [`Self`](TargetItem)."),
        (doc::side::v1::all!(), "2nd item of [`Self`](TargetItem)."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    struct TargetItem(
        /// 1st item of [`Self`].
        pub i32,
        /// 2nd item of [`Self`].
        pub i32,
    );
}

#[test]
fn test_trait() {
    #[rustfmt::skip]
   let datas = [
        (doc::base::all!(), "Document of [`Self`](TargetItem)."),
        (doc::side::CONST::all!(), "1st item of [`Self`](TargetItem)."),
        (doc::side::Type::all!(), "2nd item of [`Self`](TargetItem)."),
        (doc::side::method::all!(), "3rd item of [`Self`](TargetItem)."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    /// Document of [`Self`].
    #[doc_share(doc)]
    #[allow(unused)]
    trait TargetItem {
        /// 1st item of [`Self`].
        const CONST: i32;
        /// 2nd item of [`Self`].
        type Type;
        /// 3rd item of [`Self`].
        fn method();
    }
}

#[test]
fn test_various_self() {
    #[rustfmt::skip]
    let datas = [
        (tgt1::top!(), "Document of [Self](target1)."),
        (tgt2::top!(), "Document of [`Self`](target2)."),
        (tgt3::top!(), "Document of [`Self`](target3)."),
        (tgt4::top!(), "Document of [some function](target4)."),
        (tgt5::top!(), "Document of [some function](target5)."),
        (tgt6::base::top!(), "Document of [`Self<T>`](Target6<T>)."),
        (tgt7::base::top!(), "Document of [`Self`](array)."),
        (tgt8::base::top!(), "Document of [`Self`](fn)."),
        (tgt9::base::top!(), "Document of [`Self`](pointer)."),
        (tgt10::base::top!(), "Document of [`Self`](reference)."),
        (tgt11::base::top!(), "Document of [`Self`](slice)."),
        (tgt12::base::top!(), "Document of [`Self`](tuple)."),
        (tgt13::base::top!(), "With [`Self::item`](Target13::item)."),
        (tgt13::base::sub::cp::body!(), "With [`Self::item`](Target13::item)."),
    ];

    for (text, tobe) in datas {
        let asis = text.trim();
        assert_eq!(asis, tobe);
    }

    /// Document of [Self].
    #[doc_share(tgt1)]
    #[allow(unused)]
    fn target1() {}

    /// Document of [`Self`].
    #[doc_share(tgt2)]
    #[allow(unused)]
    fn target2() {}

    /// Document of [`Self`][].
    #[doc_share(tgt3)]
    #[allow(unused)]
    fn target3() {}

    /// Document of [some function](Self).
    #[doc_share(tgt4)]
    #[allow(unused)]
    fn target4() {}

    /// Document of [some function][link].
    ///
    /// [link]: Self
    #[doc_share(tgt5)]
    #[allow(unused)]
    fn target5() {}

    /// Document of [`Self<T>`].
    #[doc_share(tgt6)]
    #[allow(unused)]
    struct Target6<T> {
        pd: PhantomData<T>,
    }

    /// Document of [`Self`].
    #[doc_share(tgt7)]
    #[allow(unused)]
    impl SomeTrait for [i32; 3] {}

    /// Document of [`Self`].
    #[doc_share(tgt8)]
    #[allow(unused)]
    impl SomeTrait for fn() -> () {}

    /// Document of [`Self`].
    #[doc_share(tgt9)]
    #[allow(unused)]
    impl SomeTrait for *const i32 {}

    /// Document of [`Self`].
    #[doc_share(tgt10)]
    #[allow(unused)]
    impl SomeTrait for &i32 {}

    /// Document of [`Self`].
    #[doc_share(tgt11)]
    #[allow(unused)]
    impl SomeTrait for [i32] {}

    /// Document of [`Self`].
    #[doc_share(tgt12)]
    #[allow(unused)]
    impl SomeTrait for (i32, i32) {}

    /// With [`Self::item`].
    ///
    /// # cp
    ///
    /// With [`Self::item`].
    #[doc_share(tgt13)]
    #[allow(unused)]
    struct Target13 {
        item: i32,
    }

    #[allow(unused)]
    trait SomeTrait {}
}
