use clap::Parser;
use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(help = "File(s) to concatenate")]
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
    dbg!(cli);
    Ok(())
}
