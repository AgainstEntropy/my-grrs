use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use clap::Parser;
use colored::Colorize;
use log::warn;

#[derive(Parser)]
struct Cli {
    // #[arg(short, long)]
    /// The pattern to look for
    pattern: String,
    
    /// The path to the file to read
    path: std::path::PathBuf,
    
    /// The color to highlight the pattern with
    /// (default: red)
    /// (possible values: red, green, blue, yellow, magenta, cyan, white)
    #[arg(short, long, default_value = "red")]
    color: String,

    /// Show line numbers
    /// (default: false)
    #[arg(short, long)]
    line_number: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    warn!("pattern: {:?}, path: {:?}\n", args.pattern, args.path);

    let f = File::open(&args.path)
        .with_context(|| format!("could not read file `{:?}`", args.path))?;
    let reader = BufReader::new(f);

    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout);

    find_matches(reader, &args.pattern, &mut handle, args.color, args.line_number)?;

    handle.flush()?;

    Ok(())
}

fn find_matches(
    reader: BufReader<File>,
    pattern: &str,
    writer: &mut BufWriter<std::io::Stdout>,
    color: String,
    line_number: bool,
) -> Result<()> {

    let mut pline: String;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            pline = format!("{}", colorize(&line, pattern, &color));
            if line_number {
                pline = format!("{:2}: {}", index + 1, pline);
            }
            writeln!(writer, "{}", pline)?;
        }
    }

    Ok(())
}

fn colorize(line: &str, pattern: &str, color: &str) -> String {
    line.replace(pattern, &pattern.color(color).to_string())
}
