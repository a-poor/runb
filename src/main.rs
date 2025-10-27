mod app;
mod books;
mod cli;
mod conf;
mod db;
mod runner;
mod tui;

use clap::Parser;
use tokio::io::AsyncWriteExt;

use crate::cli::Cli;

#[tokio::main]
async fn main() {
    // let _cli = Cli::parse();
    // let txt = "# heading\n\nparagraph.";
    // let md = markdown::to_mdast(txt, &markdown::ParseOptions::default()).unwrap();
    // println!("{}", md.to_string());

    let md = r#"""
# Title

Paragraph

```bash
NAME="World"
echo "Hello, ${NAME}!"
```
...more...

```bash
echo 'Starting...'
curl https://example.com > /dev/null
```

The end.
"""#;
    let book = crate::books::Runbook::parse(&md).unwrap();
    for block in book.code_blocks().iter_mut() {
        let script = block.value.clone();
        let lang = block.lang.as_deref().unwrap();
        let mut child = tokio::process::Command::new("/usr/bin/env")
            .arg(lang)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .unwrap();

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(script.as_bytes()).await.unwrap();
            stdin.shutdown().await.unwrap();
        }

        let status = child.wait().await.unwrap();
        println!("Bash exited with: {}", status);
    }
}
