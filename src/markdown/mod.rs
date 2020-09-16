use std::io;
use std::io::{Error, Write};
use Escaping::{Brackets, None, Parentheses};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
pub enum Escaping {
    None,
    Brackets,
    Parentheses,
}

pub struct Markdown<W: Write> {
    writer: W,
}

impl<W: Write> Markdown<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    pub fn write<T: MarkdownWritable>(&mut self, element: T) -> Result<(), io::Error> {
        element.write_to(&mut self.writer, false, None)?;
        Ok(())
    }
}

pub trait MarkdownWritable {
    fn write_to(
        &self,
        writer: &mut dyn Write,
        inner: bool,
        escape: Escaping,
    ) -> Result<(), io::Error>;
}

//region Paragraph
pub struct Paragraph<'a> {
    children: Vec<Box<dyn 'a + MarkdownWritable>>,
}

impl<'a> Paragraph<'a> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

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
}

impl MarkdownWritable for &'_ mut Paragraph<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&**self).write_to(writer, inner, escape)
    }
}

impl MarkdownWritable for Paragraph<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&self).write_to(writer, inner, escape)
    }
}
//endregion

//region Heading
pub struct Heading<'a> {
    children: Vec<Box<dyn 'a + MarkdownWritable>>,
    level: usize,
}

impl<'a> Heading<'a> {
    pub fn new(level: usize) -> Self {
        assert!(level > 0 && level <= 6, "Heading level must be range 1-6.");
        Self {
            children: Vec::new(),
            level,
        }
    }

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
            child.write_to(writer, true, None)?;
        }
        writer.write_all(b"\n")?;
        Ok(())
    }
}

impl MarkdownWritable for &'_ mut Heading<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&**self).write_to(writer, inner, escape)
    }
}

impl MarkdownWritable for Heading<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&self).write_to(writer, inner, escape)
    }
}
//endregion

//region Link
pub struct Link<'a> {
    children: Vec<Box<dyn 'a + MarkdownWritable>>,
    address: &'a str,
}

impl<'a> Link<'a> {
    pub fn new(address: &'a str) -> Self {
        Self {
            children: Vec::new(),
            address,
        }
    }

    pub fn append<T: 'a + MarkdownWritable>(&mut self, element: T) -> &mut Self {
        self.children.push(Box::new(element));
        self
    }
}

impl MarkdownWritable for &'_ Link<'_> {
    fn write_to(
        &self,
        writer: &mut dyn Write,
        inner: bool,
        _escape: Escaping,
    ) -> Result<(), Error> {
        writer.write_all(b"[")?;
        for child in &self.children {
            child.write_to(writer, true, Brackets)?;
        }
        writer.write_all(b"](")?;
        self.address.write_to(writer, true, Parentheses)?;
        writer.write_all(b")")?;
        if !inner {
            writer.write_all(b"\n")?;
        }
        Ok(())
    }
}

impl MarkdownWritable for &'_ mut Link<'_> {
    fn write_to(
        &self,
        writer: &mut dyn Write,
        inner: bool,
        _escape: Escaping,
    ) -> Result<(), Error> {
        (&**self).write_to(writer, inner, Brackets)
    }
}

impl MarkdownWritable for Link<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        (&self).write_to(writer, inner, escape)
    }
}
//endregion

//region String and &str
impl MarkdownWritable for String {
    fn write_to(
        &self,
        writer: &mut dyn Write,
        inner: bool,
        _escape: Escaping,
    ) -> Result<(), io::Error> {
        self.as_str().write_to(writer, inner, None)
    }
}

impl MarkdownWritable for &str {
    fn write_to(&self, writer: &mut dyn Write, inner: bool, escape: Escaping) -> Result<(), Error> {
        match escape {
            None => {
                write_escaped(writer, self.as_bytes(), b"\\")?;
            }
            Brackets => {
                write_escaped(writer, self.as_bytes(), b"\\[]")?;
            }
            Parentheses => {
                write_escaped(writer, self.as_bytes(), b"\\()")?;
            }
        }
        if !inner {
            writer.write_all(b"\n\n")?;
        }
        Ok(())
    }
}
//endregion

//region AsMarkdown
pub trait AsMarkdown {
    fn as_paragraph(&self) -> Paragraph;
    fn as_heading(&self, level: usize) -> Heading;
    fn as_link_to<'a>(&'a self, address: &'a str) -> Link<'a>;
}

impl AsMarkdown for String {
    fn as_paragraph(&self) -> Paragraph {
        self.as_str().as_paragraph()
    }

    fn as_heading(&self, level: usize) -> Heading {
        self.as_str().as_heading(level)
    }

    fn as_link_to<'a>(&'a self, address: &'a str) -> Link<'a> {
        self.as_str().as_link_to(address)
    }
}

impl AsMarkdown for str {
    fn as_paragraph(&self) -> Paragraph {
        let mut p = Paragraph::new();
        p.append(self);
        p
    }

    fn as_heading(&self, level: usize) -> Heading {
        let mut h = Heading::new(level);
        h.append(self);
        h
    }

    fn as_link_to<'a>(&'a self, address: &'a str) -> Link<'a> {
        let mut l = Link::new(address);
        l.append(self);
        l
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
