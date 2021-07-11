use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short="e", long="exec")]
    command: String,
    stream: PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opt::from_args();
    let content = std::fs::read_to_string(&opts.stream)
        .with_context(|| format!("Unable to read content from file {:?}", &opts.stream))?;

    let exec_args: Vec<&str> = opts.command.split("/").collect();

    sad::handle_exec_command(&content, &exec_args);

    Ok(())
}

// Comment test test
