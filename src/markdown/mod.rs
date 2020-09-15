use std::borrow::Borrow;
use std::io::Write;
use std::io;

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

    pub fn write<E: MarkdownElement>(&mut self, element: E) -> Result<(), io::Error> {
        element.write_to(&mut self.writer)?;
        Ok(())
    }
}

pub struct Heading<S: Borrow<str>> {
    value: S,
    level: usize,
}

impl<S: Borrow<str>> Heading<S> {
    pub fn new(value: S) -> Self {
        Self {
            value,
            level: 1,
        }
    }

    pub fn level(&mut self, level: usize) -> &mut Self {
        self.level = level;
        self
    }
}

impl<S: Borrow<str>> MarkdownElement for &'_ mut Heading<S>{
    fn write_to<W: Write>(self, writer: &mut W) -> Result<(), io::Error>  where S: Borrow<str>  {
        let mut symbols = Vec::new();
        symbols.resize(self.level, '#' as u8);
        symbols.push(' ' as u8);

        writer.write_all(&symbols)?;
        writer.write_all(self.value.borrow().as_bytes())?;
        writer.write_all(&['\n' as u8])?;
        Ok(())
    }
}
impl<S: Borrow<str>> MarkdownElement for Heading<S>{
    fn write_to<W: Write>(mut self, writer: &mut W) -> Result<(), io::Error>  {
        (&mut self).write_to(writer)
    }
}

pub struct Paragraph<S: Borrow<str>> {
    value: S,
}

impl<S: Borrow<str>> Paragraph<S> {
    pub fn new(value: S) -> Self {
        Self {
            value,
        }
    }
}

impl<S: Borrow<str>> MarkdownElement for &'_ mut Paragraph<S>{
    fn write_to<W: Write>(self, writer: &mut W) -> Result<(), io::Error>  where S: Borrow<str>  {
        writer.write_all(&['\n' as u8])?;
        writer.write_all(self.value.borrow().as_bytes())?;
        writer.write_all(&['\n' as u8])?;
        Ok(())
    }
}
impl<S: Borrow<str>> MarkdownElement for Paragraph<S>{
    fn write_to<W: Write>(mut self, writer: &mut W) -> Result<(), io::Error>  {
        (&mut self).write_to(writer)
    }
}

pub trait MarkdownElement {
    fn write_to<W: Write>(self, writer: &mut W) -> Result<(), io::Error>;
}
