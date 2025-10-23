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
    let ast = markdown::to_mdast(DEMO_MD, &po).unwrap();
    print_node(&ast, 0);
}

fn print_node(node: &Node, depth: usize) {
    let prefix = " ".repeat(depth * 2);
    match node {
        Node::Root(n) => {
            println!("{}(Root)", prefix);
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Heading(n) => {
            println!("{}(Heading)", prefix);
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Paragraph(n) => {
            println!("{}(Paragraph)", prefix);
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Yaml(n) => {
            println!("{}(YAML)", prefix);
            println!("{}  {:?}", prefix, n.value);
        }
        Node::Text(n) => {
            println!("{}(Text)", prefix);
            println!("{}  {:?}", prefix, n.value);
        }
        Node::InlineCode(n) => {
            println!("{}(InlineCode)", prefix);
            println!("{}  {:?}", prefix, n.value);
        }
        Node::Strong(n) => {
            println!("{}(Bold)", prefix);
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Emphasis(n) => {
            println!("{}(Italic)", prefix);
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Delete(n) => {
            println!("{}(Strikethrough)", prefix);
            n.children.iter().for_each(|c| {
                print_node(c, depth + 1);
            });
        }
        Node::Code(n) => {
            println!("{}(Code)", prefix);
            println!(
                "{}  Type: {}",
                prefix,
                n.lang.as_deref().unwrap_or("(None)")
            );
            println!("{}  Meta: {}", prefix, parse_meta(&n.meta),);
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
