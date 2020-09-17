use std::io;
use std::io::{Error, Write};
use Escaping::{InlineCode, Normal};

#[cfg(test)]
mod tests;

/// Specifies string escaping mode
#[derive(Clone, Copy)]
pub enum Escaping {
    /// `` \`*_{}[]()#+-.!`` will be escaped with a backslash
    Normal,
    /// Inline code will be surrounded by enough backticks to escape the contents
    InlineCode,
}

/// Struct for generating Markdown
pub struct Markdown<W: Write> {
    writer: W,
}

impl<W: Write> Markdown<W> {
    /// Creates a new [Markdown](struct.Markdown.html) struct
    ///
    /// # Arguments
    ///
    /// * `writer` - Destination for Markdown data
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Returns the underlying `writer` and consumes the object
    pub fn into_inner(self) -> W {
        self.writer
    }

    /// Writes a [MarkdownWritable](trait.MarkdownWritable.html) to the document
    ///
    /// # Returns
    /// `()` or `std::io::Error` if an error occurred during writing to the underlying writer
    pub fn write<T: MarkdownWritable>(&mut self, element: T) -> Result<(), io::Error> {
        element.write_to(&mut self.writer, false, Normal)?;
        Ok(())
    }
}

/// Trait for objects writable to Markdown documents
pub trait MarkdownWritable {
    /// Writes `self` as markdown to `writer`
    ///
    /// # Arguments
    /// * `writer` - Destination writer
    /// * `inner` - `true` if element is inside another element, `false` otherwise
    /// * `mode` - Mode used for escaping string
    ///
    /// # Returns
    /// `()` or `std::io::Error` if an error occurred during writing
    fn write_to(
        &self,
        writer: &mut dyn Write,
        inner: bool,
        escape: Escaping,
    ) -> Result<(), io::Error>;

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize);
}

/// Trait for objects convertible to a Markdown element
pub trait AsMarkdown<'a> {
    /// Converts `self` to [Paragraph](struct.Paragraph.html)
    fn as_paragraph(self) -> Paragraph<'a>;
    /// Converts `self` to [Heading](struct.Heading.html)
    ///
    /// # Arguments
    /// * `level` - Heading level (1-6)
    fn as_heading(self, level: usize) -> Heading<'a>;
    /// Converts `self` to [Link](struct.Link.html)
    ///
    /// # Arguments
    /// * `address` - Address which will the link lead to
    fn as_link_to(self, address: &'a str) -> Link<'a>;
    fn as_bold(self) -> RichText<'a>;
    fn as_italic(self) -> RichText<'a>;
    fn as_code(self) -> RichText<'a>;
}

//region Paragraph
/// Markdown paragraph
pub struct Paragraph<'a> {
    children: Vec<Box<dyn 'a + MarkdownWritable>>,
}

impl<'a> Paragraph<'a> {
    /// Creates an empty paragraph
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    /// Appends an element to the paragraph
    pub fn append<T: 'a + MarkdownWritable>(&mut self, element: T) -> &mut Self {
        self.children.push(Box::new(element));
        self
    }
}

impl MarkdownWritable for &'_ Paragraph<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        assert!(!inner, "Inner paragraphs are forbidden.");
        for child in &self.children {
            child.write_to(writer, true, escape)?;
        }
        writer.write_all(b"\n\n")?;
        Ok(())
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        let mut carry = carry;
        let mut count = 0;
        for child in &self.children {
            let (c, cr) = child.count_max_streak(char, carry);
            count += c;
            carry = cr;
        }
        (count, 0)
    }
}

impl MarkdownWritable for &'_ mut Paragraph<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&**self).write_to(writer, inner, escape)
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        (&**self).count_max_streak(char, carry)
    }
}

impl MarkdownWritable for Paragraph<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&self).write_to(writer, inner, escape)
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        (&self).count_max_streak(char, carry)
    }
}
//endregion

//region Heading
/// Markdown heading
pub struct Heading<'a> {
    children: Vec<Box<dyn 'a + MarkdownWritable>>,
    level: usize,
}

impl<'a> Heading<'a> {
    /// Creates an empty heading
    ///
    /// # Arguments
    /// * `level` - Heading level (1-6)
    pub fn new(level: usize) -> Self {
        assert!(level > 0 && level <= 6, "Heading level must be range 1-6.");
        Self {
            children: Vec::new(),
            level,
        }
    }

    /// Appends an element to the heading
    pub fn append<T: 'a + MarkdownWritable>(&mut self, element: T) -> &mut Self {
        self.children.push(Box::new(element));
        self
    }
}

impl MarkdownWritable for &'_ Heading<'_> {
    fn write_to(
        &self,
        writer: &mut dyn Write,
        inner: bool,
        _escape: Escaping,
    ) -> Result<(), Error> {
        assert!(!inner, "Inner headings are forbidden.");
        let mut prefix = Vec::new();
        prefix.resize(self.level, b'#');
        prefix.push(b' ');
        writer.write_all(&prefix)?;
        for child in &self.children {
            child.write_to(writer, true, Normal)?;
        }
        writer.write_all(b"\n")?;
        Ok(())
    }

    fn count_max_streak(&self, char: u8, _carry: usize) -> (usize, usize) {
        let mut carry = 0;
        let mut count = 0;
        for child in &self.children {
            let (c, cr) = child.count_max_streak(char, carry);
            count += c;
            carry = cr;
        }
        (count, carry)
    }
}

impl MarkdownWritable for &'_ mut Heading<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&**self).write_to(writer, inner, escape)
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        (&**self).count_max_streak(char, carry)
    }
}

impl MarkdownWritable for Heading<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&self).write_to(writer, inner, escape)
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        (&self).count_max_streak(char, carry)
    }
}
//endregion

//region Link
/// Markdown link
pub struct Link<'a> {
    children: Vec<Box<dyn 'a + MarkdownWritable>>,
    address: &'a str,
}

impl<'a> Link<'a> {
    /// Creates an empty link, which leads to `address`
    pub fn new(address: &'a str) -> Self {
        Self {
            children: Vec::new(),
            address,
        }
    }

    /// Appends an element to the link's text
    pub fn append<T: 'a + MarkdownWritable>(&mut self, element: T) -> &mut Self {
        self.children.push(Box::new(element));
        self
    }
}

impl MarkdownWritable for &'_ Link<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        writer.write_all(b"[")?;
        for child in &self.children {
            child.write_to(writer, true, escape)?;
        }
        writer.write_all(b"](")?;
        self.address.write_to(writer, true, escape)?;
        writer.write_all(b")")?;
        if !inner {
            writer.write_all(b"\n")?;
        }
        Ok(())
    }

    fn count_max_streak(&self, char: u8, _carry: usize) -> (usize, usize) {
        let (addr, _) = self.address.count_max_streak(char, 0);
        let mut carry = 0;
        let mut count = 0;
        for child in &self.children {
            let (c, cr) = child.count_max_streak(char, carry);
            count += c;
            carry = cr;
        }
        return if count > addr { (count, 0) } else { (addr, 0) };
    }
}

impl MarkdownWritable for &'_ mut Link<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&**self).write_to(writer, inner, escape)
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        (&**self).count_max_streak(char, carry)
    }
}

impl MarkdownWritable for Link<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&self).write_to(writer, inner, escape)
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        (&self).count_max_streak(char, carry)
    }
}

impl<'a> AsMarkdown<'a> for &'a Link<'a> {
    fn as_paragraph(self) -> Paragraph<'a> {
        let mut p = Paragraph::new();
        p.append(self);
        p
    }

    fn as_heading(self, level: usize) -> Heading<'a> {
        let mut h = Heading::new(level);
        h.append(self);
        h
    }

    fn as_link_to(self, _address: &'a str) -> Link<'a> {
        panic!("Link cannot contain another link.");
    }

    fn as_bold(self) -> RichText<'a> {
        panic!("Cannot change link's body. Please use 'x.as_bold().as_link_to(...);'");
    }

    fn as_italic(self) -> RichText<'a> {
        panic!("Cannot change link's body. Please use 'x.as_italic().as_link_to(...);'");
    }

    fn as_code(self) -> RichText<'a> {
        panic!("Cannot change link's body. Please use 'x.as_code().as_link_to(...);'");
    }
}

impl<'a> AsMarkdown<'a> for &'a mut Link<'a> {
    fn as_paragraph(self) -> Paragraph<'a> {
        (&*self).as_paragraph()
    }

    fn as_heading(self, level: usize) -> Heading<'a> {
        (&*self).as_heading(level)
    }

    fn as_link_to(self, address: &'a str) -> Link<'a> {
        (&*self).as_link_to(address)
    }

    fn as_bold(self) -> RichText<'a> {
        (&*self).as_bold()
    }

    fn as_italic(self) -> RichText<'a> {
        (&*self).as_italic()
    }

    fn as_code(self) -> RichText<'a> {
        (&*self).as_code()
    }
}
//endregion

//region RichText
#[derive(Copy, Clone)]
pub struct RichText<'a> {
    bold: bool,
    italic: bool,
    code: bool,
    text: &'a str,
}

impl<'a> RichText<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            bold: false,
            italic: false,
            code: false,
            text,
        }
    }

    pub fn bold(&mut self) -> &mut Self {
        self.bold = true;
        self
    }

    pub fn italic(&mut self) -> &mut Self {
        self.italic = true;
        self
    }

    pub fn code(&mut self) -> &mut Self {
        self.code = true;
        self
    }
}

impl MarkdownWritable for &'_ RichText<'_> {
    fn write_to(
        &self,
        writer: &mut dyn Write,
        inner: bool,
        mut escape: Escaping,
    ) -> Result<(), Error> {
        let mut symbol = Vec::new();
        if self.bold {
            symbol.extend_from_slice(b"**");
        }
        if self.italic {
            symbol.push(b'*');
        }
        if self.code {
            let (mut ticks_needed, _) = self.text.count_max_streak(b'`', 0);
            ticks_needed += 1;
            symbol.extend(vec![b'`'; ticks_needed]);
            symbol.push(b' ');
            escape = InlineCode;
        }

        writer.write_all(&symbol)?;
        self.text.write_to(writer, true, escape)?;
        symbol.reverse();
        writer.write_all(&symbol)?;

        if !inner {
            writer.write_all(b"\n")?;
        }
        Ok(())
    }

    fn count_max_streak(&self, char: u8, _carry: usize) -> (usize, usize) {
        let (res, _) = self.text.count_max_streak(char, 0);
        (res, 0)
    }
}

impl MarkdownWritable for &'_ mut RichText<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&**self).write_to(writer, inner, escape)
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        (&**self).count_max_streak(char, carry)
    }
}

impl MarkdownWritable for RichText<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&self).write_to(writer, inner, escape)
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        (&self).count_max_streak(char, carry)
    }
}

impl<'a> AsMarkdown<'a> for &'a RichText<'a> {
    fn as_paragraph(self) -> Paragraph<'a> {
        let mut p = Paragraph::new();
        p.append(self);
        p
    }

    fn as_heading(self, level: usize) -> Heading<'a> {
        let mut h = Heading::new(level);
        h.append(self);
        h
    }

    fn as_link_to(self, address: &'a str) -> Link<'a> {
        let mut l = Link::new(address);
        l.append(self);
        l
    }

    fn as_bold(self) -> RichText<'a> {
        let mut clone = *self;
        clone.bold();
        clone
    }

    fn as_italic(self) -> RichText<'a> {
        let mut clone = *self;
        clone.italic();
        clone
    }

    fn as_code(self) -> RichText<'a> {
        let mut clone = *self;
        clone.code();
        clone
    }
}

impl<'a> AsMarkdown<'a> for &'a mut RichText<'a> {
    fn as_paragraph(self) -> Paragraph<'a> {
        (&*self).as_paragraph()
    }

    fn as_heading(self, level: usize) -> Heading<'a> {
        (&*self).as_heading(level)
    }

    fn as_link_to(self, address: &'a str) -> Link<'a> {
        (&*self).as_link_to(address)
    }

    fn as_bold(self) -> RichText<'a> {
        (&*self).as_bold()
    }

    fn as_italic(self) -> RichText<'a> {
        (&*self).as_italic()
    }

    fn as_code(self) -> RichText<'a> {
        (&*self).as_code()
    }
}
//endregion

//region String and &str
impl MarkdownWritable for &str {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        match escape {
            Normal => {
                write_escaped(writer, self.as_bytes(), b"\\`*_{}[]()#+-.!")?;
            }
            InlineCode => {
                writer.write_all(self.as_bytes())?;
            }
        }
        if !inner {
            writer.write_all(b"\n\n")?;
        }
        Ok(())
    }

    fn count_max_streak(&self, char: u8, carry: usize) -> (usize, usize) {
        let mut iter = self.as_bytes().iter();
        let mut max = 0;
        let mut current = carry;
        loop {
            match iter.next() {
                None => {
                    break;
                }
                Some(ch) => {
                    if *ch == char {
                        current += 1;
                    } else {
                        if current > max {
                            max = current;
                        }
                        current = 0;
                    }
                }
            }
        }
        (max, current)
    }
}

impl<'a> AsMarkdown<'a> for &'a String {
    fn as_paragraph(self) -> Paragraph<'a> {
        self.as_str().as_paragraph()
    }

    fn as_heading(self, level: usize) -> Heading<'a> {
        self.as_str().as_heading(level)
    }

    fn as_link_to(self, address: &'a str) -> Link<'a> {
        self.as_str().as_link_to(address)
    }

    fn as_bold(self) -> RichText<'a> {
        self.as_str().as_bold()
    }

    fn as_italic(self) -> RichText<'a> {
        self.as_str().as_italic()
    }

    fn as_code(self) -> RichText<'a> {
        self.as_str().as_code()
    }
}

impl<'a> AsMarkdown<'a> for &'a str {
    fn as_paragraph(self) -> Paragraph<'a> {
        let mut p = Paragraph::new();
        p.append(self);
        p
    }

    fn as_heading(self, level: usize) -> Heading<'a> {
        let mut h = Heading::new(level);
        h.append(self);
        h
    }

    fn as_link_to(self, address: &'a str) -> Link<'a> {
        let mut l = Link::new(address);
        l.append(self);
        l
    }

    fn as_bold(self) -> RichText<'a> {
        let mut rt = RichText::new(self);
        rt.bold();
        rt
    }

    fn as_italic(self) -> RichText<'a> {
        let mut rt = RichText::new(self);
        rt.italic();
        rt
    }

    fn as_code(self) -> RichText<'a> {
        let mut rt = RichText::new(self);
        rt.code();
        rt
    }
}
//endregion

fn write_escaped<W: Write + ?Sized>(
    writer: &mut W,
    mut data: &[u8],
    escape: &[u8],
) -> Result<(), Error> {
    loop {
        let slice_at = data.iter().position(|x| escape.contains(x));
        match slice_at {
            Option::None => {
                writer.write_all(&data)?;
                return Ok(());
            }
            Some(slice_at) => {
                writer.write_all(&data[..slice_at])?;
                writer.write_all(b"\\")?;
                writer.write_all(&data[slice_at..slice_at + 1])?;
                data = &data[slice_at + 1..];
            }
        }
    }
}
