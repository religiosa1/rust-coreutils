use std::io::{Read, Write};

use crate::base64_error::Base64Error;

pub trait Proc {
    fn proc(&mut self, input: &mut dyn Read, ouput: &mut dyn Write) -> Result<(), Base64Error>;
}
