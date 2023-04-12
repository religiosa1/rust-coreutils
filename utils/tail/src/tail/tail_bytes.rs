use super::tail_error::TailError;
use crate::args::Args;
use std::io::Read;

pub fn tail_bytes<R: Read>(_args: &Args, mut _input: R) -> Result<(), TailError> {
    todo!();
}
