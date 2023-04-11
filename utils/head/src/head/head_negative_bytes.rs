use super::head_error::HeadError;
use crate::args::Args;
use std::io::Read;

pub fn head_negative_bytes<R: Read>(_args: &Args, _input: R) -> Result<(), HeadError> {
    todo!();
}
