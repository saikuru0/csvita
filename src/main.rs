use std::error::Error;
use std::fs::File;
use clap::Parser;
use csv::{ReaderBuilder, WriterBuilder};

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
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let in_file = File::open(&args.input)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(args.flexible)
        .delimiter(args.din as u8)
        .from_reader(in_file);

    let out_file = File::create(&args.output)?;
    let mut writer = WriterBuilder::new()
        .flexible(args.flexible)
        .quote_style(csv::QuoteStyle::Always)
        .delimiter(args.dout as u8)
        .double_quote(!(args.escape))
        .from_writer(out_file);

    for result in reader.records() {
        let record = result?;
        let _ = writer.write_record(&record);
    }

    writer.flush()?;
    Ok(())
}
