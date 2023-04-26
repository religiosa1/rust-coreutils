mod split_regex;
mod split_str;

use split_regex::split_regex;
use split_str::split_str;
use std::io::Read;

use crate::{args::Args, entry::Entry, tac_error::TacError};

pub fn split<R: Read>(args: &Args, input: &mut R) -> Result<Vec<Entry>, TacError> {
    match args.regex {
        true => split_regex(input, &args.separator),
        false => split_str(input, &args.separator),
    }
}
