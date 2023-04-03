#![allow(unused)]
use std::fmt::format;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
    let content = std::fs::read_to_string(&args.path).expect("Could not open file");
    // println!("file content {}", content);

    find_matches(&content, &args.pattern);
}

fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
