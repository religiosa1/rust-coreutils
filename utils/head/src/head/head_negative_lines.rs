use crate::args::Args;
use std::io::Read;

use super::head_error::HeadError;

pub fn head_negative_lines<R: Read>(args: &Args, input: R) -> Result<(), HeadError> {
    let lines_to_omit = args.lines.to_usize().ok_or(HeadError::Overflow)?;

    Ok(())
}
