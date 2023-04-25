use crate::{
    alphabet::{Alphabet, RFC4648_ALPHABET},
    args::Args,
    proc::Proc,
};
use std::io::{Read, Write};

pub struct Decoder;
impl Decoder {
    pub fn new(_args: &Args) -> Decoder {
        Decoder
    }
}

impl Proc for Decoder {
    fn proc(
        &mut self,
        _input: &mut dyn Read,
        _write: &mut dyn Write,
    ) -> Result<(), std::io::Error> {
        todo!();
    }
}

fn decode_chunk(alphabet: &Alphabet, chunk: &[u8], output: &mut [u8]) -> Result<usize, String> {
    let mut n: usize = 0;
    let b0 = chunk.get(0).and_then(|b| alphabet.value(*b).unwrap());
    let b1 = chunk.get(1).and_then(|b| alphabet.value(*b).unwrap());
    let b2 = chunk.get(2).and_then(|b| alphabet.value(*b).unwrap());
    let b3 = chunk.get(3).and_then(|b| alphabet.value(*b).unwrap());
    let b4 = chunk.get(4).and_then(|b| alphabet.value(*b).unwrap());
    let b5 = chunk.get(5).and_then(|b| alphabet.value(*b).unwrap());
    let b6 = chunk.get(6).and_then(|b| alphabet.value(*b).unwrap());
    let b7 = chunk.get(7).and_then(|b| alphabet.value(*b).unwrap());

    if let Some(b) = b0 {
        output[0] = b << 3 | (b1.unwrap_or(0) & 0b11100) >> 2;
        n = 1;
    }
    if let Some(b) = b1 {
        let val = b & 0b11;
        if val != 0 || matches!(b2, Some(_)) {
            output[1] = val << 6 | b2.unwrap_or(0) << 1 | b3.unwrap_or(0) & 0b10000 >> 4;
            n = 2;
        }
    }
    if let Some(b) = b3 {
        let val = b & 0b1111;
        if val != 0 || matches!(b4, Some(_)) {
            output[2] = val << 4 | (b4.unwrap_or(0) & 0b11110) >> 1;
            n = 3;
        }
    }
    if let Some(b) = b4 {
        let val = b & 0b1;
        if val != 0 || matches!(b5, Some(_)) {
            output[3] = val << 7 | b5.unwrap_or(0) << 2 | (b6.unwrap_or(0) & 0b11000) >> 3;
            n = 4;
        }
    }
    if let Some(b) = b6 {
        let val = b & 0b111;
        if val != 0 || matches!(b7, Some(_)) {
            output[4] = val << 5 | b7.unwrap_or(0);
            n = 5;
        }
    }

    Ok(n)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn decode_chunk_works_as_expected() {
        let mut output = [0_u8; 5];
        let b = decode_chunk(&RFC4648_ALPHABET, b"ME======", &mut output);
        assert_eq!(output.to_vec(), b"a\0\0\0\0".to_vec());
        assert_eq!(b.unwrap(), 1);

        let b = decode_chunk(&RFC4648_ALPHABET, b"MFRA====", &mut output);
        assert_eq!(output.to_vec(), b"ab\0\0\0".to_vec());
        assert_eq!(b.unwrap(), 2);

        let b = decode_chunk(&RFC4648_ALPHABET, b"MFRGG===", &mut output);
        assert_eq!(output.to_vec(), b"abc\0\0".to_vec());
        assert_eq!(b.unwrap(), 3);

        let b = decode_chunk(&RFC4648_ALPHABET, b"MFRGGZA=", &mut output);
        assert_eq!(output.to_vec(), b"abcd\0".to_vec());
        assert_eq!(b.unwrap(), 4);

        let b = decode_chunk(&RFC4648_ALPHABET, b"MFRGGZDF", &mut output);
        assert_eq!(output.to_vec(), b"abcde".to_vec());
        assert_eq!(b.unwrap(), 5);

        let b = decode_chunk(&RFC4648_ALPHABET, b"", &mut output);
        assert_eq!(b.unwrap(), 0);
    }
}
