use clap::Parser;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::string::String;
use std::fs::File;

#[derive(Parser, Debug)]
struct CliArguments {
    pattern: String,
    path: std::path::PathBuf, 
}

fn main() {
    let args = CliArguments::parse();

    let file = File::open(&args.path).expect("couldn't read the file");
    let reader = BufReader::new(file);

    let result: Vec<String> = reader
        .lines()
        .filter(|x: &Result<String, Error>| x.as_ref().unwrap().contains(&args.pattern))
        .map(|x| x.unwrap()).collect();

    println!("{:?}", result);
    println!("{:?}", args);
}
