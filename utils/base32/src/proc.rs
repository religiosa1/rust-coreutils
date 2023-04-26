use std::io::{Read, Write};

use crate::base32_error::Base32Error;

pub trait Proc {
    fn proc(&mut self, input: &mut dyn Read, ouput: &mut dyn Write) -> Result<(), Base32Error>;
}
