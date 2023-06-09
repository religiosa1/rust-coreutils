mod args;
mod proc;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use args::Args;
use chunk_reader::ChunkReader;
use proc::ProcessorDirector;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut processor = ProcessorDirector::new(&args);
    for file in args.file {
        cat(&file, &mut processor)?;
    }
    Ok(())
}

fn cat(file: &str, processor: &mut ProcessorDirector) -> io::Result<()> {
    let input: Box<dyn Read> = match file {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(file)?),
    };

    let reader = BufReader::new(input);

    for line in reader.chunks(b'\n', 8192) {
        if let Some(processed_line) = processor.proc(line?) {
            io::stdout().write(&processed_line)?;
        }
    }
    Ok(())
}
