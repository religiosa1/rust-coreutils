use crate::args::Args;
use parse_num::NumValue;

mod head_bytes;
pub mod head_error;
mod head_lines;
mod head_negative_bytes;
mod head_negative_lines;
use head_bytes::head_bytes;
use head_lines::head_lines;
use head_negative_bytes::head_negative_bytes;
use head_negative_lines::head_negative_lines;

use head_error::HeadError;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

pub fn head(args: &Args, name: &str) -> Result<(), HeadError> {
    let input: Box<dyn Read> = match name {
        "-" => Box::new(io::stdin()),
        _ => {
            let file = File::open(name).map_err(|e| HeadError::Io(e))?;
            Box::new(file)
        }
    };
    let reader = BufReader::new(input);
    let mut output = BufWriter::new(std::io::stdout());

    match args {
        Args {
            bytes: Some(NumValue {
                prefix: Some('-'), ..
            }),
            ..
        } => head_negative_bytes(args, reader, &mut output),
        Args { bytes: Some(_), .. } => head_bytes(args, reader, &mut output),
        Args {
            lines: NumValue {
                prefix: Some('-'), ..
            },
            ..
        } => head_negative_lines(args, reader, &mut output),
        _ => head_lines(args, reader, &mut output),
    }
}
