use anyhow::{anyhow, Context, Error, Result};
use clap_verbosity_flag::Verbosity;
use grrs::{print_matches, purge_file, write_matches};
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

    /// The path to the output file to write to
    #[structopt(short, long, parse(from_os_str))]
    outfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = Cli::from_args();
    let path = &args.path;
    let pattern = &args.pattern;
    let outfile = &args.outfile;

    match pattern.trim().is_empty() {
        false => Some(pattern),
        true => return Err(anyhow!("pattern appears to be empty")),
    };
    let f =
        File::open(path).with_context(|| format!("could not read file `{}`", path.display()))?;
    let reader = BufReader::new(f);
    let mut line_num = 0;

    match outfile {
        None => {
            for line in reader.lines() {
                line_num += 1;
                print_matches(&line?, &line_num, pattern, &mut std::io::stdout())?;
            }
        }
        Some(outfile) => {
            purge_file(outfile)
                .with_context(|| format!("could not create file '{}'", outfile.display()))?;
            for line in reader.lines() {
                line_num += 1;
                write_matches(&line?, &line_num, pattern, outfile)?;
            }
        }
    }

    Ok(())
}
