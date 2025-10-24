//! Code for parsing / working with runbooks.

use markdown::mdast::Node;
use serde::{Deserialize, Serialize};

pub struct Runbook {
    mdast_node: Node,
}

/// The schema for a runbook's frontmatter.
#[derive(Debug, Clone, Deserialize)]
pub struct BookFrontmatter;

///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeBlockMeta;
