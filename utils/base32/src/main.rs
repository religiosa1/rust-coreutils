mod args;
mod decode;
mod encode;

use args::Args;
use decode::decode;
use encode::encode;
use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let args = Args::parse();
    let action = if args.decode { decode } else { encode };
    for filename in &args.file {
        let file: Box<dyn Read> = match filename.as_str() {
            "-" => Box::new(std::io::stdin()),
            _ => {
                let file = File::open(filename)?;
                Box::new(file)
            }
        };
        action(&args, file)?;
    }
    Ok(())
}
