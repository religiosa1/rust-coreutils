use crate::{args::Args, proc::Proc};
use std::io::{Read, Result, Write};

pub struct Decoder;
impl Decoder {
    pub fn new(_args: &Args) -> Decoder {
        Decoder
    }
}

impl Proc for Decoder {
    fn proc(&mut self, _input: &mut dyn Read, _write: &mut dyn Write) -> Result<()> {
        todo!();
    }
}
