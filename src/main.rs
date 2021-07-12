use std::path::PathBuf;
use clap::Clap;
use anyhow::{Context, Result};

#[derive(Clap, Debug)]
struct Opts {
    /// Text script to be executed on file
    script: String,
    /// File path to execute the script on
    file: PathBuf,
    /// Edit file in place or make a backup if a suffix is provided
    #[clap(short, long)]
    in_place: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    println!("{}", opts.in_place);
    let content = std::fs::read_to_string(&opts.file)
        .with_context(|| format!("Unable to read content from file {:?}", &opts.file))?;

    let exec_args: Vec<&str> = opts.script.split("/").collect();

    sad::handle_exec_command(&content, &exec_args);

    Ok(())
}

// Comment test test
