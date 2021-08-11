use anyhow::{anyhow, Context, Result};
use clap_verbosity_flag::Verbosity;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let path = &args.path;
    let pattern = &args.pattern;
    match pattern.trim().is_empty() {
        false => pattern,
        true => return Err(anyhow!("pattern appears to be empty")),
    };
    let f =
        File::open(path).with_context(|| format!("could not read file `{}`", path.display()))?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        grrs::find_matches(&line?, pattern, &mut std::io::stdout());
    }

    Ok(())
}
