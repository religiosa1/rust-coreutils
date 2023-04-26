mod args;
mod decode;
mod encode;
mod filtered_reader;
mod proc;

use args::Args;
use decode::Decoder;
use encode::Encoder;
use proc::Proc;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Result},
};

fn main() -> Result<()> {
    let args = Args::parse();
    let mut p: Box<dyn Proc> = match args.decode {
        true => Box::new(Decoder::new(&args)),
        false => Box::new(Encoder::new(&args)),
    };
    for filename in &args.file {
        let file: Box<dyn Read> = match filename.as_str() {
            "-" => Box::new(std::io::stdin()),
            _ => {
                let file = File::open(filename)?;
                Box::new(file)
            }
        };
        let mut input = BufReader::new(file);
        p.proc(&mut input, &mut BufWriter::new(std::io::stdout()))?;
    }
    Ok(())
}
