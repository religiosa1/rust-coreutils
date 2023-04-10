use crate::args::Args;
use std::io::{Read, Result};

pub fn head_bytes<R: Read>(_args: &Args, _input: R) -> Result<()> {
    todo!();
}
