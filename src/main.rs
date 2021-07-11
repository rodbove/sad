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

    for line in content.lines() {
        if line.contains(exec_args[1]) {
            let new_line = line.replace(exec_args[1], exec_args[2]);
            println!("{}", new_line);
        } else {
            println!("{}", line);
        }
    }

    println!("{:?}", opts);
    Ok(())
}
