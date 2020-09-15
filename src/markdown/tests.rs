use super::Markdown;
use crate::markdown::AsMarkdown;

#[test]
fn test() {
    let mut md = Markdown::new(Vec::new());
    md.write(String::from("heading1").as_heading(1)).unwrap();
    md.write("heading2".as_heading(2).append(" appended")).unwrap();
    md.write("first\nparagraph".as_paragraph().append(" appended")).unwrap();
    md.write("second\nparagraph").unwrap();
    md.write(String::from("third\nparagraph")).unwrap();
    assert_eq!(
    String::from_utf8(md.into_inner()).unwrap(),
    "# heading1\n\
    ## heading2 appended\n\
    \n\
    first\n\
    paragraph appended\n\
    \n\
    second\n\
    paragraph\n\
    \n\
    third\n\
    paragraph\n"
    );
}
