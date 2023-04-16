use super::tail_error::TailError;
use crate::args::Args;
use std::io::{Read, Write};

use chunk_reader::ChunkReader;
use ibig::UBig;

const CHUNK_SIZE: usize = 8192;

pub fn tail_negative_lines<R: Read>(args: &Args, input: R) -> Result<(), TailError> {
    let lines_to_omit = args.lines.to_ubig();
    let mut counter = UBig::from(0_usize);

    for chunk in input.chunks(args.terminator, CHUNK_SIZE) {
        let chunk = chunk?;
        if counter < lines_to_omit {
            if chunk.ends_with(&[args.terminator]) {
                counter += 1;
            }
        } else {
            std::io::stdout().write(&chunk)?;
        }
    }
    std::io::stdout().flush()?;
    Ok(())
}
