use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
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
    let content = std::fs::read_to_string(&opts.file)
        .with_context(|| format!("Unable to read content from file {:?}", &opts.file))?;

    let exec_args: Vec<&str> = opts.script.split("/").collect();

    let result = sad::handle_exec_command(&content, &exec_args).unwrap();

    if opts.in_place {
        let mut file = File::create(&opts.file).unwrap();
        if let Err(e) = writeln!(file, "{}", result) {
            eprintln!("Error trying to write result to file: {}", e);
        }
    }

    Ok(())
}

// Comment test test
