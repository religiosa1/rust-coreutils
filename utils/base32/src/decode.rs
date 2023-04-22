use crate::{args::Args, proc::Proc};
use std::io::{Read, Result, Write};

pub struct Decoder;
impl Decoder {
    pub fn new(args: &Args) -> Decoder {
        Decoder
    }
}

impl Proc for Decoder {
    fn proc(&mut self, input: &mut dyn Read, write: &mut dyn Write) -> Result<()> {
        todo!();
    }
}
