use super::Markdown;
use crate::markdown::{AsMarkdown, List};

//region Heading
#[test]
fn headings() {
    let mut md = Markdown::new(Vec::new());

    md.write("h1".heading(1)).unwrap();
    md.write("h2".heading(2)).unwrap();
    md.write("h3".heading(3)).unwrap();
    md.write("h4".heading(4)).unwrap();
    md.write("h5".heading(5)).unwrap();
    md.write("h6".heading(6)).unwrap();

    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "# h1\n\
        ## h2\n\
        ### h3\n\
        #### h4\n\
        ##### h5\n\
        ###### h6\n"
    );
}

#[test]
#[should_panic]
fn panic_on_inner_heading() {
    let mut md = Markdown::new(Vec::new());

    md.write("h1".paragraph().append("this should panic".heading(1)))
        .unwrap();
}

#[test]
fn heading_append() {
    let mut md = Markdown::new(Vec::new());
    md.write("h1".heading(1).append("appended")).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "# h1appended\n"
    );
}
//endregion

//region Paragraph
#[test]
fn paragraphs() {
    let mut md = Markdown::new(Vec::new());

    md.write("test paragraph".paragraph()).unwrap();
    md.write("test paragraph2".paragraph()).unwrap();

    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "test paragraph\n\
        \n\
        test paragraph2\n\
        \n"
    );
}

#[test]
fn paragraph_append() {
    let mut md = Markdown::new(Vec::new());

    md.write("test paragraph".paragraph().append(" append"))
        .unwrap();

    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "test paragraph append\n\
        \n"
    );
}
//endregion

//region String
#[test]
fn string() {
    let mut md = Markdown::new(Vec::new());
    md.write("test string").unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "test string\n\
        \n"
    );
}

#[test]
fn string_escaping() {
    let mut md = Markdown::new(Vec::new());
    md.write("\\test\\string\\").unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "\\\\test\\\\string\\\\\n\
        \n"
    );
}
//endregion

//region Link
#[test]
fn link() {
    let mut md = Markdown::new(Vec::new());
    md.write("test link".link_to("https://test.url")).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "[test link](https://test\\.url)\n"
    );
}

#[test]
fn link_escaping() {
    let mut md = Markdown::new(Vec::new());
    md.write("[][]test [] link[][]".link_to("https://test().url()"))
        .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "[\\[\\]\\[\\]test \\[\\] link\\[\\]\\[\\]](https://test\\(\\)\\.url\\(\\))\n"
    );
}

#[test]
fn link_append() {
    let mut md = Markdown::new(Vec::new());
    md.write(
        "test link"
            .link_to("https://test.url")
            .append(" ")
            .append("appended"),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "[test link appended](https://test\\.url)\n"
    );
}
//endregion

//region RichText
#[test]
fn code() {
    let mut md = Markdown::new(Vec::new());
    md.write("co`````de".code()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "`````` co`````de ``````\n\n"
    );
}

#[test]
fn bold() {
    let mut md = Markdown::new(Vec::new());
    md.write("bo****ld".bold()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "**bo\\*\\*\\*\\*ld**\n\n"
    );
}

#[test]
fn italic() {
    let mut md = Markdown::new(Vec::new());
    md.write("ita**lic".italic()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "*ita\\*\\*lic*\n\n"
    );
}

#[test]
fn bold_italic() {
    let mut md = Markdown::new(Vec::new());
    md.write("bold italic".italic().bold()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "***bold italic***\n\n"
    );

    let mut md = Markdown::new(Vec::new());
    md.write("bold italic".bold().italic()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "***bold italic***\n\n"
    );
}

#[test]
fn bold_italic_code() {
    let mut md = Markdown::new(Vec::new());
    md.write("bold italic code".italic().bold().code()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "***` bold italic code `***\n\n"
    );

    let mut md = Markdown::new(Vec::new());
    md.write("bold italic".bold().italic()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "***bold italic***\n\n"
    );
}

#[test]
fn asterisk_escaping() {
    let mut md = Markdown::new(Vec::new());
    md.write("test **".bold()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "**test \\*\\***\n\n"
    );
}
//endregion

//region List
#[test]
fn list() {
    let mut md = Markdown::new(Vec::new());
    md.write(
        List::new(true).item("item 1").item("bold".bold()).item(
            List::new(false)
                .title("nested list")
                .item("bold".bold().paragraph().append("italic".italic())),
        ),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "\n   1. item 1\n   1. **bold**\n   1. nested list\n      * **bold***italic*"
    );
}
//endregion

//region Quote
#[test]
fn quote() {
    let mut md = Markdown::new(Vec::new());
    md.write("bold quote".bold().quote()).unwrap();
    md.write("code quote".code().quote()).unwrap();
    md.write("test ".quote().append("link".link_to("sample.url")))
        .unwrap();
    md.write(
        List::new(true)
            .title("quoted list")
            .item("item")
            .item(
                List::new(false)
                    .title("nested quoted list")
                    .item("bold item quote".bold().quote())
                    .item("test".link_to("sample.url")),
            )
            .quote(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "\n>**bold quote**\n\n\n>` code quote `\n\n\n>test [link](sample\\.url)\n\n\n>quoted list\n>   1. item\n>   1. nested quoted list\n>      * >**bold item quote**\n>      * [test](sample\\.url)\n\n"
    );
}
//endregion

//region Other
#[test]
fn link_as_heading() {
    let mut md = Markdown::new(Vec::new());
    md.write("test link".link_to("https://test.url").heading(2))
        .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "## [test link](https://test\\.url)\n"
    );
}

#[test]
fn bold_link() {
    let mut md = Markdown::new(Vec::new());
    md.write("bold link".bold().link_to("https://bold"))
        .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "[**bold link**](https://bold)\n"
    );
}

#[test]
fn as_functions_do_not_move() {
    let str = String::from("test");
    str.paragraph();
    str.heading(1);
    str.link_to("test");
    str.bold();
    str.italic();
    assert_eq!(str, "test");
}

#[test]
fn unicode() {
    let mut md = Markdown::new(Vec::new());
    md.write(
        "뜲漜ֵٰ𷸞ڡ򬻵y콰񍋋ȱ擥񲇧ۼ򠝊₧☾y굻瘲놶􋄻ᘝmā򞛥~ݳ奂ҳu"
            .bold()
            .italic()
            .code(),
    )
    .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "***` 뜲漜ֵٰ𷸞ڡ򬻵y콰񍋋ȱ擥񲇧ۼ򠝊₧☾y굻瘲놶􋄻ᘝmā򞛥~ݳ奂ҳu `***\n\n"
    );
}
//endregion
