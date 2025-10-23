use markdown::ParseOptions;
use markdown::mdast::Node;

const DEMO_MD: &str = r#"---
name: "Foo"
---
# My Heading

This is a paragraph that starts here
and continues along here.

This is the 2nd paragraph.

```json
{"foo": "bar", "baz": 42"}
```

Mid-way paragraph.

```json title="foo.json" linenums
{"foo": "bar", "baz": 42"}
```
What about this?

``` {.python title="foo.json" linenums}
{"foo": "bar", "baz": 42"}
```
Last paragraph.

"#;

fn main() {
    let mut po = ParseOptions::gfm();
    po.constructs.frontmatter = true;
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
                n.meta.as_deref().unwrap_or("(None)")
            );
            println!("{}  Content: {:?}", prefix, n.value);
        }
        _ => {}
    }
}
