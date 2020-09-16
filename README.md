# markdown-gen
Rust crate for generating Markdown files
## Usage
```rust
 let file = File::create("test.md").unwrap();
 let mut md = Markdown::new(file);

 md.write("Heading".as_heading(1)).unwrap();
 md.write("Subheading".as_heading(2)).unwrap();

 md.write("first paragraph").unwrap();

 md.write(
    "Links: ".as_paragraph()
        .append("Rust".as_link_to("https://rust-lang.org"))
        .append(", ")
        .append("Google".as_link_to("https://google.com"))
 ).unwrap();
```
This produces the following Markdown document
```
# Heading
## Subheading
first paragraph

Links: [Rust](https://rust-lang.org), [Google](https://google.com)
```

You can also generate Markdown to `Vec<u8>`:
```
let mut md = Markdown::new(Vec::new());

md.write("test".as_heading(1)).unwrap();

let vec = md.into_inner();
assert_eq!(String::from_utf8(vec).unwrap(), "# test\n");
```
