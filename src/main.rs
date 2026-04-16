use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::Command;
use std::string::String;
use std::usize;

#[derive(Parser, Debug)]
struct CliArguments {
    pattern: String,
    path: std::path::PathBuf,
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let args = CliArguments::parse();

    env_logger::Builder::new().filter_level(args.verbosity.into()).init();
    info!("Starting up Gureppu");

    // let stdout = io::stdout();
    // let mut handle = stdout.lock();

    let file = File::open(&args.path)
        .with_context(|| format!("couldn't read file: {}", &args.path.display()))?;

    let reader = BufReader::new(file);

    let result: Vec<(usize, String)> = reader
        .lines()
        .enumerate()
        .filter(|(_, x)| x.as_ref().unwrap().contains(&args.pattern))
        .map(|(i, x)| (i, x.unwrap()))
        .collect();

    let pb = indicatif::ProgressBar::new(result.len().try_into().unwrap());
    for line in result.iter() {
        let mut child = Command::new("sleep").arg("0.5").spawn().unwrap();
        let _result = child.wait().unwrap();
        pb.println(format!("{} {}", (line.0 + 1), line.1));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    info!("Finish Gureppu");

    Ok(())
}
