use std::io;
use std::io::{Error, Write};
#[cfg(test)]
mod tests;

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
        element.write_to(&mut self.writer, false)?;
        Ok(())
    }
}

pub trait MarkdownWritable {
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), io::Error>;
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
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        assert!(!inner);
        writer.write_all(b"\n")?;
        for child in &self.children {
            child.write_to(writer, true)?;
        }
        writer.write_all(b"\n")?;
        Ok(())
    }
}

impl MarkdownWritable for &'_ mut Paragraph<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        (&**self).write_to(writer, inner)
    }
}

impl MarkdownWritable for Paragraph<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        (&self).write_to(writer, inner)
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
        assert!(level > 0 && level <= 6);
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
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        assert!(!inner);
        let mut prefix = Vec::new();
        prefix.resize(self.level, b'#');
        prefix.push(b' ');
        writer.write_all(&prefix)?;
        for child in &self.children {
            child.write_to(writer, true)?;
        }
        writer.write_all(b"\n")?;
        Ok(())
    }
}

impl MarkdownWritable for &'_ mut Heading<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        (&**self).write_to(writer, inner)
    }
}

impl MarkdownWritable for Heading<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        (&self).write_to(writer, inner)
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
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        if !inner {
            writer.write_all(b"\n")?;
        }
        writer.write_all(b"[")?;
        for child in &self.children {
            child.write_to(writer, true)?;
        }
        writer.write_all(b"](")?;
        writer.write_all(self.address.as_bytes())?;
        writer.write_all(b")")?;
        Ok(())
    }
}

impl MarkdownWritable for &'_ mut Link<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        (&**self).write_to(writer, inner)
    }
}

impl MarkdownWritable for Link<'_> {
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        (&self).write_to(writer, inner)
    }
}
//endregion

//region String and &str
impl MarkdownWritable for String {
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), io::Error> {
        self.as_str().write_to(writer, inner)
    }
}

impl MarkdownWritable for &str {
    fn write_to(&self, writer: &mut dyn Write, inner: bool) -> Result<(), Error> {
        if !inner {
            writer.write_all(b"\n")?;
        }
        writer.write_all(self.as_bytes())?;
        if !inner {
            writer.write_all(b"\n")?;
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
