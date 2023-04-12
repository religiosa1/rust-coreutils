use crate::args::Args;
use parse_num::NumValue;

mod tail_bytes;
pub mod tail_error;
mod tail_lines;
mod tail_negative_bytes;
mod tail_negative_lines;
use tail_bytes::tail_bytes;
use tail_lines::tail_lines;
use tail_negative_bytes::tail_negative_bytes;
use tail_negative_lines::tail_negative_lines;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use tail_error::TailError;

pub fn tail(args: &Args, name: &str) -> Result<(), TailError> {
    let input: Box<dyn Read> = match name {
        "-" => Box::new(io::stdin()),
        _ => {
            let file = File::open(name).map_err(|e| TailError::Io(e))?;
            Box::new(file)
        }
    };
    let reader = BufReader::new(input);

    match args {
        Args {
            bytes: Some(NumValue {
                prefix: Some('+'), ..
            }),
            ..
        } => tail_negative_bytes(args, reader),
        Args { bytes: Some(_), .. } => tail_bytes(args, reader),
        Args {
            lines: NumValue {
                prefix: Some('+'), ..
            },
            ..
        } => tail_negative_lines(args, reader),
        _ => tail_lines(args, reader),
    }
}
