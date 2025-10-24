mod app;
mod books;
mod cli;
mod conf;
mod db;
mod runner;
mod tui;

use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();
}
