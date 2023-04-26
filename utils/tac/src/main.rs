mod args;
mod entry;
mod split;
mod tac;
mod tac_error;

use std::{
    fs::File,
    io::{BufReader, Read},
};

use args::Args;
use tac::tac;
use tac_error::TacError;

fn main() -> Result<(), TacError> {
    let args = Args::parse();
    for filename in &args.file {
        let file: Box<dyn Read> = match filename.as_str() {
            "-" => Box::new(std::io::stdin()),
            _ => Box::new(File::open(filename)?),
        };
        let mut reader = BufReader::new(file);
        tac(&args, &mut reader, &mut std::io::stdout())?;
    }
    Ok(())
}
