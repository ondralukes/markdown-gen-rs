use super::Markdown;
use crate::markdown::AsMarkdown;

//region Heading
#[test]
fn headings() {
    let mut md = Markdown::new(Vec::new());

    md.write("h1".as_heading(1)).unwrap();
    md.write("h2".as_heading(2)).unwrap();
    md.write("h3".as_heading(3)).unwrap();
    md.write("h4".as_heading(4)).unwrap();
    md.write("h5".as_heading(5)).unwrap();
    md.write("h6".as_heading(6)).unwrap();

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

    md.write(
        "h1".as_paragraph()
            .append("this should panic".as_heading(1)),
    )
    .unwrap();
}

#[test]
fn heading_append() {
    let mut md = Markdown::new(Vec::new());
    md.write("h1".as_heading(1).append("appended")).unwrap();
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

    md.write("test paragraph".as_paragraph()).unwrap();
    md.write("test paragraph2".as_paragraph()).unwrap();

    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "test paragraph\n\
        \n\
        test paragraph2\n\
        \n"
    );
}

#[test]
#[should_panic]
fn panic_on_inner_paragraph() {
    let mut md = Markdown::new(Vec::new());

    md.write(
        "h1".as_paragraph()
            .append("this should panic".as_paragraph()),
    )
    .unwrap();
}

#[test]
fn paragraph_append() {
    let mut md = Markdown::new(Vec::new());

    md.write("test paragraph".as_paragraph().append(" append"))
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
    md.write("test link".as_link_to("https://test.url"))
        .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "[test link](https://test\\.url)\n"
    );
}

#[test]
fn link_escaping() {
    let mut md = Markdown::new(Vec::new());
    md.write("[][]test [] link[][]".as_link_to("https://test().url()"))
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
            .as_link_to("https://test.url")
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
    md.write("co`````de".as_code()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "`````` co`````de ``````\n"
    );
}

#[test]
fn bold() {
    let mut md = Markdown::new(Vec::new());
    md.write("bo****ld".as_bold()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "**bo\\*\\*\\*\\*ld**\n"
    );
}

#[test]
fn italic() {
    let mut md = Markdown::new(Vec::new());
    md.write("ita**lic".as_italic()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "*ita\\*\\*lic*\n"
    );
}

#[test]
fn bold_italic() {
    let mut md = Markdown::new(Vec::new());
    md.write("bold italic".as_italic().bold()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "***bold italic***\n"
    );

    let mut md = Markdown::new(Vec::new());
    md.write("bold italic".as_bold().italic()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "***bold italic***\n"
    );
}

#[test]
fn bold_italic_code() {
    let mut md = Markdown::new(Vec::new());
    md.write("bold italic code".as_italic().bold().code())
        .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "***` bold italic code `***\n"
    );

    let mut md = Markdown::new(Vec::new());
    md.write("bold italic".as_bold().italic()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "***bold italic***\n"
    );
}

#[test]
fn asterisk_escaping() {
    let mut md = Markdown::new(Vec::new());
    md.write("test **".as_bold()).unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "**test \\*\\***\n"
    );
}
//endregion

//region Other
#[test]
fn link_as_heading() {
    let mut md = Markdown::new(Vec::new());
    md.write("test link".as_link_to("https://test.url").as_heading(2))
        .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "## [test link](https://test\\.url)\n"
    );
}

#[test]
fn bold_link() {
    let mut md = Markdown::new(Vec::new());
    md.write("bold link".as_bold().as_link_to("https://bold"))
        .unwrap();
    assert_eq!(
        String::from_utf8(md.into_inner()).unwrap(),
        "[**bold link**](https://bold)\n"
    );
}

#[test]
fn as_functions_do_not_move() {
    let str = String::from("test");
    str.as_paragraph();
    str.as_heading(1);
    str.as_link_to("test");
    str.as_bold();
    str.as_italic();
    assert_eq!(str, "test");
}
//endregion
