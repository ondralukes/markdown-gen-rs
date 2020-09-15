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

pub struct Heading<'a> {
    value: StringBuffer<'a>,
    level: usize,
}

impl<'a> Heading<'a> {
    pub fn new(value: &'a str) -> Self {
        let mut buf = StringBuffer::new();
        buf.append(value);
        Self {
            value: buf,
            level: 1,
        }
    }

    pub fn append(&mut self, value: &'a str) -> &mut Self{
        self.value.append(value);
        self
    }

    pub fn level(&mut self, level: usize) -> &mut Self {
        self.level = level;
        self
    }
}

impl MarkdownElement for &'_ mut Heading<'_>{
    fn write_to<W: Write>(self, writer: &mut W) -> Result<(), io::Error>  {
        let mut symbols = Vec::new();
        symbols.resize(self.level, '#' as u8);
        symbols.push(' ' as u8);

        writer.write_all(&symbols)?;
        self.value.write_to(writer)?;
        writer.write_all(&['\n' as u8])?;
        Ok(())
    }
}
impl MarkdownElement for Heading<'_>{
    fn write_to<W: Write>(mut self, writer: &mut W) -> Result<(), io::Error>  {
        (&mut self).write_to(writer)
    }
}

pub struct Paragraph<'a> {
    value: StringBuffer<'a>,
}

impl<'a> Paragraph<'a> {
    pub fn new(value: &'a str) -> Self {
        let mut buf = StringBuffer::new();
        buf.append(value);
        Self {
            value: buf
        }
    }

    pub fn append(&mut self, value: &'a str) -> &mut Self{
        self.value.append(value);
        self
    }
}

impl MarkdownElement for &'_ mut Paragraph<'_>{
    fn write_to<W: Write>(self, writer: &mut W) -> Result<(), io::Error>  {
        writer.write_all(&['\n' as u8])?;
        self.value.write_to(writer)?;
        writer.write_all(&['\n' as u8])?;
        Ok(())
    }
}
impl MarkdownElement for Paragraph<'_>{
    fn write_to<W: Write>(mut self, writer: &mut W) -> Result<(), io::Error>  {
        (&mut self).write_to(writer)
    }
}

pub trait MarkdownElement {
    fn write_to<W: Write>(self, writer: &mut W) -> Result<(), io::Error>;
}

struct StringBuffer<'a>{
    strings: Vec<&'a str>
}

impl<'a> StringBuffer<'a>{
    fn new() -> Self{
        Self{
            strings: Vec::new()
        }
    }

    fn append(&mut self, str: &'a str) {
        self.strings.push(str);
    }

    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), io::Error>{
        for s in &self.strings{
            writer.write_all(s.as_bytes())?;
        }
        Ok(())
    }
}
