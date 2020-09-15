use super::{Markdown, Heading, Paragraph};

#[test]
fn test() {
    let mut md = Markdown::new(Vec::new());
    md.write(Heading::new("heading1")).unwrap();
    md.write(Heading::new("heading2").level(2)).unwrap();
    md.write(Paragraph::new("first\nparagraph")).unwrap();
    md.write(Paragraph::new("second\nparagraph")).unwrap();
    assert_eq!(
    String::from_utf8(md.into_inner()).unwrap(),
    "# heading1\n\
    ## heading2\n\
    \n\
    first\n\
    paragraph\n\
    \n\
    second\n\
    paragraph\n"
    );
}
