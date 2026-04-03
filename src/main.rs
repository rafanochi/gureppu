use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::string::String;

#[derive(Parser, Debug)]
struct CliArguments {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = CliArguments::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("couldn't read file: {}", &args.path.display()))?;

    let reader = BufReader::new(file);

    let result: Vec<String> = reader
        .lines()
        .filter(|x: &Result<String, Error>| x.as_ref().unwrap().contains(&args.pattern))
        .map(|x| x.unwrap())
        .collect();

    println!("{:?}", result);
    println!("{:?}", args);

    Ok(())
}
