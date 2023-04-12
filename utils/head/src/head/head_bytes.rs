use crate::args::Args;
use ibig::{ops::DivRem, UBig};
use std::io::{Read, Write};

const CHUNK_SIZE: usize = 8192;

use super::head_error::HeadError;

pub fn head_bytes<R: Read>(args: &Args, mut input: R) -> Result<(), HeadError> {
    let mut buffer = [0_u8; CHUNK_SIZE];
    let max = args.bytes.clone().unwrap().to_ubig();
    let (n_full_chunks, partial_read) = max.div_rem(CHUNK_SIZE);
    let mut counter = UBig::from(0_usize);

    if n_full_chunks > UBig::from(0_usize) {
        while counter < n_full_chunks {
            let bytes_read = input.read(&mut buffer)?;
            counter += 1;
            if bytes_read == 0 {
                return Ok(());
            }
            std::io::stdout().write(&buffer[..bytes_read])?;
        }
    }
    if partial_read > 0 {
        let bytes_read = input.read(&mut buffer[..partial_read])?;
        if bytes_read == 0 {
            return Ok(());
        }
        std::io::stdout().write(&buffer[..bytes_read])?;
    }
    Ok(())
}
