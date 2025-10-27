//! Code for parsing / working with runbooks.

use anyhow::{Result, anyhow};
use markdown::mdast::{Code, Node, Root};
use serde::{Deserialize, Serialize};
// use std::path::PathBuf;

pub struct Runbook {
    /// Stores the root markdown ast node
    mdroot: Root,
}

impl Runbook {
    ///
    pub fn parse(raw: &str) -> Result<Self> {
        // Parse the markdown
        let po = markdown::ParseOptions::gfm();
        let md = match markdown::to_mdast(raw, &po) {
            Ok(ast) => ast,
            Err(err) => return Err(anyhow!("Failed to parse md: {}", err.reason)),
        };

        // Get the root node
        let root = match md {
            Node::Root(root) => root,
            _ => return Err(anyhow!("the root md node wasn't a root: {:?}", md)),
        };

        // Anything else?
        // ...

        // Good
        Ok(Self { mdroot: root })
    }

    pub fn code_blocks(&self) -> Vec<Code> {
        self.mdroot
            .children
            .iter()
            .filter_map(|node| {
                let x = match node {
                    Node::Code(c) => Some(c.clone()),
                    _ => None,
                };
                x
            })
            .collect::<Vec<_>>()
    }
}

/// The structure of a runbook's frontmatter.
#[derive(Debug, Clone, Deserialize)]
pub struct BookFrontmatter {}

/// The structure of a runbook markdown code block's
/// meta section -- which is stored as json.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeBlockMeta {}
