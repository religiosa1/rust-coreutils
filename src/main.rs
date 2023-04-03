mod args;
mod proc;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use args::Args;
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
    let input: Box<dyn Read> = if file != "-" {
        Box::new(File::open(file)?)
    } else {
        Box::new(io::stdin())
    };

    let reader = BufReader::new(input);

    for line in reader.lines() {
        if let Some(processed_line) = processor.proc(line?) {
            println!("{}", processed_line);
        }
    }
    Ok(())
}
