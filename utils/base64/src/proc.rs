use std::io::{Read, Result, Write};

pub trait Proc {
    fn proc(&mut self, input: &mut dyn Read, ouput: &mut dyn Write) -> Result<()>;
}
