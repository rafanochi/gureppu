use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
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

    let mut line = String::new();

    for mut line in reader.lines() {
         if (&mut line).as_mut().expect("Couldn't read the LINE").contains(&args.pattern) {
            println!("{}", line.unwrap());
        }
    }

    println!("{:?}", args);
}




