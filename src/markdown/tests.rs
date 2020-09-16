use super::Markdown;
use crate::markdown::AsMarkdown;

#[test]
fn test() {
    let mut md = Markdown::new(Vec::new());
    md.write(String::from("heading1").as_heading(1)).unwrap();
    md.write("heading2".as_heading(2).append(" appended"))
        .unwrap();
    md.write("first\nparagraph".as_paragraph().append(" appended"))
        .unwrap();
    md.write("second\nparagraph".as_paragraph().append(
        "Rust [Wikipedia]".as_link_to("https://en.wikipedia.org/wiki/Rust_(programming_language)"),
    ))
    .unwrap();
    md.write(String::from("third\nparagraph")).unwrap();
    md.write("google".as_link_to("https://google.com")).unwrap();
    let string = String::from_utf8(md.into_inner()).unwrap();
    println!("{}", string);
    assert_eq!(
        string,
        "# heading1\n\
    ## heading2 appended\n\
    \n\
    first\n\
    paragraph appended\n\
    \n\
    second\n\
    paragraph[Rust \\[Wikipedia\\]](https://en.wikipedia.org/wiki/Rust_\\(programming_language\\))\n\
    \n\
    third\n\
    paragraph\n\
    \n\
    [google](https://google.com)"
    );
}
