use crate::args::Args;
use chunk_reader::ChunkReader;
use ibig::UBig;
use std::io::{Read, Write};

use super::head_error::HeadError;

pub fn head_lines<R: Read>(args: &Args, input: R) -> Result<(), HeadError> {
    let mut counter = UBig::from(0_usize);
    let max = args.lines.to_ubig();
    for chunk in input.chunks(b'\n', 8192) {
        if counter >= max {
            break;
        }
        let chunk = chunk?;
        std::io::stdout().write(&chunk)?;

        if chunk.ends_with(b"\n") {
            counter += 1;
        }
    }
    Ok(())
}
