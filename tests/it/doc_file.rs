use rustdoc_copy::prelude::*;

#[test]
fn simple() {
    use doc::sub::sample as fst;
    doc_file!(doc, "md/simple.md");

    let datas = [
        (doc::head!(), ""),
        (doc::body!(), ""),
        (doc::top!(), ""),
        (fst::head!(), "# Sample"),
        (fst::body!(), "This is sample."),
        (fst::top!(), "# Sample\n\nThis is sample."),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe);
    }
}

#[test]
fn with_cmark_features() {
    use doc::sub::footnotes as ft;
    use doc::sub::image_definitions as id;
    use doc::sub::link_definitions as ld;
    use doc::sub::smart_punctuations as sp;
    use doc::sub::strike_through as st;
    use doc::sub::tables as ta;
    use doc::sub::task_lists as tl;
    doc_file!(doc, "md/cmark_features.md#");

    #[rustfmt::skip]
    let datas = [
        (ld::body!(), "[link](http://www.example.com/)."),
        (id::body!(), "![image](http://www.example.com/img.png)"),
        (st::body!(), "~~deleted~~."),
        (sp::body!(), "“quoted”."),
        (tl::body!(), "* [ ] Task1\n* [x] Task2"),
        (ta::body!(), "|Head1|Head2|\n|-----|-----|\n|Cell1|Cell2|"),
        (ft::body!(), "Some text[^1].\n\n[^1]: http://www.example.com/"),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe);
    }
}

#[test]
fn with_root_fragment() {
    doc_file!(doc, "md/simple.md#");
    let datas = [
        (doc::head!().trim(), ""),
        (doc::body!().trim(), "This is sample."),
        (doc::top!().trim(), "This is sample."),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe);
    }
}

#[test]
fn with_unofficial_header() {
    doc_file!(doc, "md/unofficial_header.md#");
    let datas = [
        (doc::head!().trim(), ""),
        (doc::body!().trim(), "This is sample."),
        (doc::top!().trim(), "This is sample."),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe);
    }
}

#[test]
fn with_multi_sections() {
    use doc::sub::section_a as doc_a;
    use doc::sub::section_a::sub::section_x as doc_ax;
    use doc::sub::section_a::sub::section_y as doc_ay;
    use doc::sub::section_b as doc_b;
    use doc::sub::section_b::sub::section_x as doc_bx;
    use doc::sub::section_b::sub::section_y as doc_by;
    doc_file!(doc, "md/multi_sections.md#");

    #[rustfmt::skip]
    let datas = [
        (doc_a::head!(), vec!["# Section A"]),
        (doc_b::head!(), vec!["# Section B"]),
        (doc_ax::head!(), vec!["## Section x"]),
        (doc_ay::head!(), vec!["## Section y"]),
        (doc_a::body!(), vec!["Section A."]),
        (doc_b::body!(), vec!["Section B."]),
        (doc_ax::body!(), vec!["Section A-x."]),
        (doc_ay::body!(), vec!["Section A-y."]),
        (doc_a::top!(), vec![doc_a::head!(), doc_a::body!()]),
        (doc_b::top!(), vec![doc_b::head!(), doc_b::body!()]),
        (doc_ax::top!(), vec![doc_ax::head!(), doc_ax::body!()]),
        (doc_ay::top!(), vec![doc_ay::head!(), doc_ay::body!()]),
        (doc_bx::top!(), vec![doc_bx::head!(), doc_bx::body!()]),
        (doc_by::top!(), vec![doc_by::head!(), doc_by::body!()]),
        (doc_a::all!(), vec![doc_a::top!(), doc_ax::all!(), doc_ay::all!()]),
        (doc_b::all!(), vec![doc_b::top!(), doc_bx::all!(), doc_by::all!()]),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe.join("\n\n").trim());
    }
}

#[test]
fn with_parts() {
    use doc::sub::sub1;
    use doc::sub::sub2;
    doc_file!(doc, "md/parts.md");

    let datas = [
        (doc::head!(), vec!["Root 1st."]),
        (doc::body!(), vec!["Root 2nd.\n\nRoot 3rd."]),
        (doc::defs!(), vec!["[link]: http://www.example.com/"]),
        (doc::subs!(), vec![sub1::all!(), sub2::all!()]),
        (doc::top!(), vec![doc::head!(), doc::body!()]),
        (doc::all!(), vec![doc::top!(), doc::subs!(), doc::defs!()]),
        (sub1::head!(), vec!["# Sub1"]),
        (sub1::body!(), vec!["Sub1 1st.\n\nSub1 2nd.\n\nSub1 3rd."]),
        (sub1::subs!(), vec![]),
        (sub1::top!(), vec![sub1::head!(), sub1::body!()]),
        (sub1::all!(), vec![sub1::top!(), sub1::subs!()]),
        (sub2::head!(), vec!["# Sub2"]),
        (sub2::body!(), vec!["Sub2 1st.\n\nSub2 2nd.\n\nSub2 3rd."]),
        (sub2::subs!(), vec![]),
        (sub2::top!(), vec![sub2::head!(), sub2::body!()]),
        (sub2::all!(), vec![sub2::top!(), sub2::subs!()]),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe.join("\n\n").trim());
    }
}

#[test]
fn with_common_titles_in_sec() {
    use doc::sub::common_title as sec0;
    use doc::sub::common_title_1 as sec1;
    use doc::sub::common_title_2 as sec2;
    doc_file!(doc, "md/common_titles_in_sec.md#");

    #[rustfmt::skip]
    let datas = [
        (sec0::head!(), vec!["# Common title"]),
        (sec1::head!(), vec!["# Common title"]),
        (sec2::head!(), vec!["# Common title"]),
        (sec0::body!(), vec!["Section 0."]),
        (sec1::body!(), vec!["Section 1."]),
        (sec2::body!(), vec!["Section 2."]),
        (doc::all!(), vec![doc::top!(), sec0::all!(), sec1::all!(), sec2::all!()]),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe.join("\n\n").trim());
    }
}

#[test]
fn with_common_titles_in_doc() {
    doc_file!(sec_a_sub, "md/common_titles_in_doc.md#common-title");
    doc_file!(sec_b_sub, "md/common_titles_in_doc.md#common-title-1");
    doc_file!(sec_c_sub, "md/common_titles_in_doc.md#common-title-2");

    let datas = [
        (sec_a_sub::head!(), "# Common title"),
        (sec_b_sub::head!(), "# Common title"),
        (sec_c_sub::head!(), "# Common title"),
        (sec_a_sub::body!(), "Section A sub."),
        (sec_b_sub::body!(), "Section B sub."),
        (sec_c_sub::body!(), "Section C sub."),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe);
    }
}

#[test]
fn with_links() {
    use doc::sub;
    doc_file!(doc, "md/links.md#");

    #[rustfmt::skip]
    let datas = [
        (sub::inline::body!(), "[link](http://www.example.com/)"),
        (sub::shortcut::body!(), "[link-sc](http://www.example.com/)"),
        (sub::collapsed::body!(), "[link-cl](http://www.example.com/)"),
        (sub::reference::body!(), "[link-ref](http://www.example.com/)"),
        (sub::shortcut_unknown::body!(), "[link-sc-unknown]"),
        (sub::collapsed_unknown::body!(), "[link-cl-unknown][]"),
        (sub::reference_unknown::body!(), "[link-ref-unknown][link-ref-unknown]"),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe);
    }
}

#[test]
fn with_copy_guard() {
    use doc::sub;
    doc_file!(doc, "md/copy_guard.md#");

    #[rustfmt::skip]
    let datas = [
        (sub::inline::body!(), "[link]"),
        (sub::shortcut::body!(), "[link-sc]"),
        (sub::reference::body!(), "[link-ref][link-ref]"),
    ];

    for (asis, tobe) in datas {
        assert_eq!(asis.trim(), tobe);
    }
}
