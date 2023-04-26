mod alphabet;
mod args;
mod decode;
mod encode;
mod proc;

use args::Args;
use decode::Decoder;
use encode::Encoder;
use proc::Proc;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut p: Box<dyn Proc> = if args.decode {
        Box::new(Decoder::new(&args))
    } else {
        Box::new(Encoder::new(&args))
    };
    for filename in &args.file {
        let file: Box<dyn Read> = match filename.as_str() {
            "-" => Box::new(std::io::stdin()),
            _ => {
                let file = File::open(filename)?;
                Box::new(file)
            }
        };
        let mut reader = BufReader::new(file);
        p.proc(&mut reader, &mut BufWriter::new(std::io::stdout()))?;
    }
    Ok(())
}
