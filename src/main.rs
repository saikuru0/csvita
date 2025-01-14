use clap::Parser;
use csv::ReaderBuilder;
use std::fs::File;
use std::{error::Error, io::Write};

#[derive(Parser, Debug)]
#[command(version, about = "CSV cleanup and reformat tool", long_about = None)]
struct Args {
    /// Input CSV filepath
    #[arg(short, long)]
    input: String,

    /// Output CSV filepath
    #[arg(short, long)]
    output: String,

    /// Delimiter to read
    #[arg(long, default_value = ",")]
    din: char,

    /// Delimiter to write
    #[arg(long, default_value = ",")]
    dout: char,

    /// Escape with backslash instead of double-quoting
    #[arg(short, long)]
    escape: bool,

    /// Flexible read and write (doesn't skip rows with column count mismatch)
    #[arg(short, long)]
    flexible: bool,

    /// Skips using quotes for empty cells
    #[arg(long)]
    skip_empty: bool,

    /// Skips using quotes for number cells
    #[arg(long)]
    skip_nums: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let in_file = File::open(&args.input)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(args.flexible)
        .delimiter(args.din as u8)
        .from_reader(in_file);

    let mut out_file = File::create(&args.output)?;

    for result in reader.records() {
        let record = result?;
        writeln!(
            out_file,
            "{}",
            record
                .iter()
                .map(|x| {
                    if args.skip_empty && x.is_empty() {
                        return "".to_string();
                    }
                    if args.skip_nums && x.parse::<i128>().is_ok() {
                        return x.to_string();
                    }
                    format!(
                        "\"{}\"",
                        x.to_string().replace(
                            "\"",
                            (|esc| {
                                if esc {
                                    "\\\""
                                } else {
                                    "\"\""
                                }
                            })(args.escape)
                        )
                    )
                })
                .collect::<Vec<String>>()
                .join(&args.dout.to_string())
        )?;
    }

    Ok(())
}
