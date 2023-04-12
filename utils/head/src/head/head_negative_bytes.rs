use super::head_error::HeadError;
use crate::args::Args;
use std::io::{Read, Write};

pub fn head_negative_bytes<R: Read>(args: &Args, mut input: R) -> Result<(), HeadError> {
    let bytes_to_omit = args
        .bytes
        .as_ref()
        .unwrap()
        .to_usize()
        .ok_or(HeadError::Overflow)?;
    let mut buffer: Vec<u8> = Vec::new();
    input.read_to_end(&mut buffer)?;
    if bytes_to_omit < buffer.len() {
        std::io::stdout().write(&buffer[..buffer.len() - bytes_to_omit])?;
    }
    Ok(())
}
