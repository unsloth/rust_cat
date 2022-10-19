use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = "Concatenate FILE(s) to standard output.
With no FILE, or when FILE is -, read standard output"
)]
pub struct Cli {
    #[arg(default_values = vec!["-"], help = "File(s) to concatenate")]
    files: Vec<String>,

    #[arg(short, long, help = "number all output lines")]
    number: bool,

    #[arg(short = 'b', long, help = "number nonempty output lines, overrides -n")]
    number_nonblank: bool,
}

pub fn get_args() -> MyResult<Cli> {
    Ok(Cli::parse())
}

pub fn run(cli: Cli) -> MyResult<()> {
    for filename in cli.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(text) => read_text(text)?,
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_text(text: Box<dyn BufRead>) -> MyResult<()> {
    for line in text.lines() {
        println!("{}", line?);
    }
    Ok(())
}
