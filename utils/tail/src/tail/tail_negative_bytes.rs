use super::tail_error::TailError;
use crate::args::Args;
use chunk_reader::ChunkReader;
use ibig::{ops::DivRem, UBig};
use std::io::{Read, Write};

const CHUNK_SIZE: usize = 8192;

pub fn tail_negative_bytes<R: Read>(args: &Args, mut input: R) -> Result<(), TailError> {
    let bytes_to_omit = args.bytes.clone().unwrap().to_ubig();
    let (n_full_chunks, partial_read) = bytes_to_omit.clone().div_rem(CHUNK_SIZE);
    let mut counter = UBig::from(0_usize);

    if bytes_to_omit > UBig::from(0_usize) {
        let mut buffer = [0_u8; CHUNK_SIZE];
        while counter < n_full_chunks {
            input.read(&mut buffer)?;
            counter += 1;
        }
        if partial_read > 0 {
            input.read(&mut buffer[..partial_read])?;
        }
    }
    for chunk in input.chunks(args.terminator, CHUNK_SIZE) {
        let chunk = chunk?;
        std::io::stdout().write(&chunk)?;
    }
    std::io::stdout().flush()?;
    Ok(())
}
