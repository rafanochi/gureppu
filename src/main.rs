use anyhow::{Context, Result};
use clap::Parser;
use std::fmt::format;
use std::fs::File;
use std::io::{self, prelude::*, stdout};
use std::io::{BufReader, Error};
use std::string::String;
use std::usize;

#[derive(Parser, Debug)]
struct CliArguments {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = CliArguments::parse();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let file = File::open(&args.path)
        .with_context(|| format!("couldn't read file: {}", &args.path.display()))?;

    let reader = BufReader::new(file);

    let result: Vec<(usize, String)> = reader
        .lines()
        .enumerate()
        .filter(|(_, x)| x.as_ref().unwrap().contains(&args.pattern))
        .map(|(i,x)| (i, x.unwrap()))
        .collect();

    for line in result.iter()  {
       writeln!(handle, "{} {}", (line.0 + 1), line.1); 
    }

    Ok(())
}
