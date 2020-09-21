# markdown-gen
Rust crate for generating Markdown files
## Usage
```rust
let file = File::create("test.md").unwrap();
let mut md = Markdown::new(file);

md.write("Heading".heading(1)).unwrap();
md.write("Subheading".italic().heading(2)).unwrap();

md.write("bold".bold()).unwrap();

md.write("first paragraph").unwrap();
md.write(
    "Links: ".paragraph()
    .append("Rust".bold().link_to("https://rust-lang.org"))
    .append(", ")
    .append("Google".italic().link_to("https://google.com"))
).unwrap();

md.write(
    List::new(true)
        .title("numbered list")
        .item("item 1")
        .item("bold".bold())
        .item(
                List::new(false)
                    .title("nested bullet list")
                    .item(
                        "bold".bold()
                            .paragraph().append(
                            "italic".italic()
                        )
                    )
           )
).unwrap();
```
This produces the following Markdown document
```
# Heading
## *Subheading*
**bold**

first paragraph

Links: [**Rust**](https://rust\-lang\.org), [*Google*](https://google\.com)

numbered list
   1. item 1
   1. **bold**
   1. nested bullet list
      * **bold***italic*
```

You can also generate Markdown to `Vec<u8>`:
```rust
let mut md = Markdown::new(Vec::new());

md.write("test".heading(1)).unwrap();

let vec = md.into_inner();
assert_eq!(String::from_utf8(vec).unwrap(), "# test\n");
```
