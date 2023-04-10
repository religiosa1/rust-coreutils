use crate::{args::Args, parse_num::NumValue};

mod head_bytes;
mod head_lines;
mod head_negative_bytes;
mod head_negative_lines;
use head_bytes::head_bytes;
use head_lines::head_lines;
use head_negative_bytes::head_negative_bytes;
use head_negative_lines::head_negative_lines;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn head(args: &Args, name: &str) -> io::Result<()> {
    let input: Box<dyn Read> = match name {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(name)?),
    };
    let reader = BufReader::new(input);

    match args {
        Args {
            bytes: Some(NumValue {
                prefix: Some('-'), ..
            }),
            ..
        } => head_negative_bytes(args, reader),
        Args { bytes: Some(_), .. } => head_bytes(args, reader),
        Args {
            lines: NumValue {
                prefix: Some('-'), ..
            },
            ..
        } => head_negative_lines(args, reader),
        _ => head_lines(args, reader),
    }
}
