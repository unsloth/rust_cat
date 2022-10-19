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

pub fn run() -> MyResult<()> {
    let cli = Cli::parse();

    if cli.number_nonblank {
        read_text_b(cli.files)?
    } else if cli.number {
        read_text_n(cli.files)?
    } else {
        read_text(cli.files)?
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_text(files: Vec<String>) -> MyResult<()> {
    for filename in files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(text) => {
                for line in text.lines() {
                    println!("{}", line?);
                }
            }
        }
    }
    Ok(())
}

fn read_text_n(files: Vec<String>) -> MyResult<()> {
    let mut c = 1;
    for filename in files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(text) => {
                for line in text.lines() {
                    println!("{:>6}\t{}", c, line?);
                    c += 1;
                }
            }
        }
    }
    Ok(())
}

fn read_text_b(files: Vec<String>) -> MyResult<()> {
    let mut c = 1;
    for filename in files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(text) => {
                for line in text.lines() {
                    let line = line?;
                    if line == "" {
                        println!("{}", line)
                    } else {
                        println!("{:>6}\t{}", c, line);
                        c += 1;
                    }
                }
            }
        }
    }
    Ok(())
}
