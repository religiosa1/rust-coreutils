use crate::{
    alphabet::{Alphabet, RFC4648_ALPHABET},
    args::Args,
    proc::Proc,
};
use std::error::Error;
use std::io::{Read, Write};

const BUF_SIZE: usize = 8;
const OUTPUT_BUF_SIZE: usize = 5;

pub struct Decoder {
    buf: [u8; BUF_SIZE],
    output_buf: [u8; OUTPUT_BUF_SIZE],
    ignore_garbage: bool,
}
impl Decoder {
    pub fn new(args: &Args) -> Decoder {
        Decoder {
            buf: [0_u8; BUF_SIZE],
            output_buf: [0_u8; OUTPUT_BUF_SIZE],
            ignore_garbage: args.ignore_garbage,
        }
    }
}

impl Proc for Decoder {
    fn proc(&mut self, input: &mut dyn Read, output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let mut temp = [0_u8; 1];
        let filter_fn = match self.ignore_garbage {
            true => |b: u8| matches!(RFC4648_ALPHABET.value(b), Ok(_)),
            false => |b: u8| !b.is_ascii_whitespace(),
        };
        let mut bytes_read: usize = 0;
        loop {
            let n = input.read(&mut temp)?;

            if n != 0 && filter_fn(temp[0]) {
                self.buf[bytes_read] = temp[0];
                bytes_read += 1;
            }
            if bytes_read == BUF_SIZE || n == 0 {
                let bytes_conv = decode_chunk(
                    &RFC4648_ALPHABET,
                    &self.buf[..bytes_read],
                    &mut self.output_buf,
                )?;
                bytes_read = 0;
                output.write(&self.output_buf[..bytes_conv])?;
            }
            if n == 0 {
                break;
            }
        }

        Ok(())
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
            output[1] = val << 6 | b2.unwrap_or(0) << 1 | (b3.unwrap_or(0) & 0b10000) >> 4;
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

    #[test]
    fn additional_tests_for_decode_chunk() {
        let mut output = [0_u8; OUTPUT_BUF_SIZE];
        let n = decode_chunk(&RFC4648_ALPHABET, b"NZQW2ZI=", &mut output).unwrap();
        let str = String::from_utf8(output[..n].to_vec()).unwrap();
        assert_eq!(str, String::from("name"));
    }
}
