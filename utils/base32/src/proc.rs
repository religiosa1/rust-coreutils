use std::{
    error::Error,
    io::{Read, Write},
};

pub trait Proc {
    fn proc(&mut self, input: &mut dyn Read, ouput: &mut dyn Write) -> Result<(), Box<dyn Error>>;
}
