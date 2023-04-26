use crate::{args::Args, entry::Entry, split::split, tac_error::TacError};
use std::io::{Read, Write};

pub fn tac<R: Read, W: Write>(args: &Args, input: &mut R, output: &mut W) -> Result<(), TacError> {
    let entries: Vec<Entry> = split(args, input)?;

    for e in entries.iter().rev() {
        match args.before {
            true => {
                output.write(e.separator.as_slice())?;
                output.write(e.line.as_slice())?;
            }
            false => {
                output.write(e.line.as_slice())?;
                output.write(e.separator.as_slice())?;
            }
        }
    }
    Ok(())
}
