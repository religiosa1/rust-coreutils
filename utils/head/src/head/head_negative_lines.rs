use crate::args::Args;
use chunk_reader::ChunkReader;
use std::io::{Read, Write};

use super::head_error::HeadError;

pub fn head_negative_lines<R: Read>(args: &Args, input: R) -> Result<(), HeadError> {
    let terminator = if args.zero_terminated { b'\0' } else { b'\n' };
    let lines_to_omit = args.lines.to_usize().ok_or(HeadError::Overflow)?;
    let lines: Vec<Vec<u8>> = input
        .chunks(terminator, 0)
        .collect::<Result<Vec<Vec<u8>>, _>>()?;

    if lines.len() > lines_to_omit {
        for line in &lines[..lines.len() - lines_to_omit] {
            std::io::stdout().write(line)?;
        }
    }
    Ok(())
}
