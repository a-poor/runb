mod app;
mod cli;
mod conf;
mod db;
mod runner;
mod tui;

use markdown::ParseOptions;
use markdown::mdast::Node;

const DEMO_MD: &str = r#"---
name: "Foo"
---
# My Heading

This is a paragraph that starts here
and continues along here.

This is the 2nd paragraph.

```json {"id": "001", "runnable": true}
{"foo": "bar", "baz": 42"}
```

Last paragraph with *italic*, **bold**, `code`,
and ~strikethrough~.

"#;

fn main() {
    let mut po = ParseOptions::gfm();
    po.constructs.frontmatter = true;
    // po.constructs.gfm_strikethrough = true;
    let ast = markdown::to_mdast(&DEMO_MD, &po).unwrap();
    print_node(&ast, 0);
}

fn print_node(node: &Node, depth: usize) {
    let prefix = " ".repeat(depth * 2);
    match node {
        Node::Root(n) => {
            println!("{}{}", prefix, "(Root)");
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Heading(n) => {
            println!("{}{}", prefix, "(Heading)");
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Paragraph(n) => {
            println!("{}{}", prefix, "(Paragraph)");
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Yaml(n) => {
            println!("{}{}", prefix, "(YAML)");
            println!("{}  {:?}", prefix, n.value);
        }
        Node::Text(n) => {
            println!("{}{}", prefix, "(Text)");
            println!("{}  {:?}", prefix, n.value);
        }
        Node::InlineCode(n) => {
            println!("{}{}", prefix, "(InlineCode)");
            println!("{}  {:?}", prefix, n.value);
        }
        Node::Strong(n) => {
            println!("{}{}", prefix, "(Bold)");
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Emphasis(n) => {
            println!("{}{}", prefix, "(Italic)");
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Delete(n) => {
            println!("{}{}", prefix, "(Strikethrough)");
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Code(n) => {
            println!("{}{}", prefix, "(Code)");
            println!(
                "{}  Type: {}",
                prefix,
                n.lang.as_deref().unwrap_or("(None)")
            );
            println!(
                "{}  Meta: {}",
                prefix,
                // n.meta.as_deref().unwrap_or("(None)")
                parse_meta(&n.meta),
            );
            println!("{}  Content: {:?}", prefix, n.value);
        }
        _ => {}
    }
}

fn parse_meta(meta: &Option<String>) -> String {
    match meta {
        None => "(None)".to_string(),
        Some(s) => match serde_json::from_str::<serde_json::Value>(s) {
            Ok(val) => format!("{:?}", val),
            Err(_err) => format!("(Not parsed) {}", s),
        },
    }
}
