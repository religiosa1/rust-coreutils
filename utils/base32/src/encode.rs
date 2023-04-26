use crate::alphabet::{Alphabet, RFC4648_ALPHABET};
use crate::base32_error::Base32Error;
use crate::{args::Args, proc::Proc};
use std::io::{Read, Write};

const BUF_SIZE: usize = 5;
const OUT_BUF_SIZE: usize = ((BUF_SIZE / 5 * 8 + 7) & !7) * 2;

pub struct Encoder {
    buf: [u8; BUF_SIZE],
    output_buf: [u8; OUT_BUF_SIZE],
    wrap: usize,
}

impl Encoder {
    pub fn new(args: &Args) -> Encoder {
        let wrap: usize = args.wrap.try_into().unwrap();
        Encoder {
            buf: [0_u8; BUF_SIZE],
            output_buf: [0_u8; OUT_BUF_SIZE],
            wrap: wrap,
        }
    }
}

impl Proc for Encoder {
    fn proc(&mut self, input: &mut dyn Read, output: &mut dyn Write) -> Result<(), Base32Error> {
        let mut writer = wrapped_writer::WrappedWriter::new(output, self.wrap);
        let mut bytes_read_in_chunk = 0;
        loop {
            let bytes_read = input.read(&mut self.buf[bytes_read_in_chunk..])?;
            bytes_read_in_chunk += bytes_read;
            if bytes_read_in_chunk == 5 || bytes_read == 0 {
                let bytes_converted = encode_chunk(
                    &RFC4648_ALPHABET,
                    &self.buf[..bytes_read_in_chunk],
                    &mut self.output_buf,
                )?;
                writer.write(&self.output_buf[..bytes_converted])?;
                bytes_read_in_chunk = 0;
            }
            if bytes_read == 0 {
                break;
            }
        }
        Ok(())
    }
}

fn encode_chunk(
    alphabet: &Alphabet,
    chunk: &[u8],
    output: &mut [u8],
) -> Result<usize, Base32Error> {
    let b0 = match chunk.get(0).copied() {
        Some(b) => b,
        None => return Ok(0),
    };
    let b1 = chunk.get(1).copied();
    let b2 = chunk.get(2).copied();
    let b3 = chunk.get(3).copied();
    let b4 = chunk.get(4).copied();

    output[0] = alphabet.symbol((b0 & 0b1111_1000) >> 3)?;
    output[1] =
        alphabet.symbol((b0 & 0b0000_0111) << 2 | (b1.unwrap_or(0_u8) & 0b1100_0000) >> 6)?;
    let mut bytes_written = 2;

    if let Some(b) = b1 {
        output[2] = alphabet.symbol((b & 0b0011_1110) >> 1)?;
        output[3] =
            alphabet.symbol((b & 0b0000_0001) << 4 | (b2.unwrap_or(0) & 0b1111_0000) >> 4)?;
        bytes_written = 4;
    }
    if let Some(b) = b2 {
        output[4] = alphabet.symbol((b & 0b1111) << 1 | (b3.unwrap_or(0) & 0b1000_0000) >> 7)?;
        bytes_written = 5;
    }
    if let Some(b) = b3 {
        output[5] = alphabet.symbol((b & 0b0111_1100) >> 2)?;
        output[6] = alphabet.symbol((b & 0b11) << 3 | (b4.unwrap_or(0) & 0b1110_0000) >> 5)?;
        bytes_written = 7;
    }
    if let Some(b) = b4 {
        output[7] = alphabet.symbol(b & 0b1_1111)?;
        bytes_written = 8;
    }

    if bytes_written < 8 {
        if let Some(p) = alphabet.padding {
            output[bytes_written..8].fill(p);
            bytes_written = 8;
        }
    }

    Ok(bytes_written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn encode_chunk_behaves_as_expected() {
        let mut output = [0_u8; 8];
        let b = encode_chunk(&RFC4648_ALPHABET, b"a", &mut output);
        assert_eq!(output.to_vec(), b"ME======".to_vec());
        assert_eq!(b.unwrap(), 8);

        let b = encode_chunk(&RFC4648_ALPHABET, b"ab", &mut output);
        assert_eq!(output.to_vec(), b"MFRA====".to_vec());
        assert_eq!(b.unwrap(), 8);

        let b = encode_chunk(&RFC4648_ALPHABET, b"abc", &mut output);
        assert_eq!(output.to_vec(), b"MFRGG===".to_vec());
        assert_eq!(b.unwrap(), 8);

        let b = encode_chunk(&RFC4648_ALPHABET, b"abcd", &mut output);
        assert_eq!(output.to_vec(), b"MFRGGZA=".to_vec());
        assert_eq!(b.unwrap(), 8);

        let b = encode_chunk(&RFC4648_ALPHABET, b"abcde", &mut output);
        assert_eq!(output.to_vec(), b"MFRGGZDF".to_vec());
        assert_eq!(b.unwrap(), 8);

        let b = encode_chunk(&RFC4648_ALPHABET, b"", &mut output);
        assert_eq!(b.unwrap(), 0);
    }

    #[test]
    fn encode_without_padding_return_the_expected_amount_of_bytes() {
        let mut output = [0_u8; 8];
        let mut alph = RFC4648_ALPHABET.clone();
        alph.padding = None;
        let b = encode_chunk(&alph, b"a", &mut output);
        assert_eq!(output.to_vec(), b"ME\0\0\0\0\0\0".to_vec());
        assert_eq!(b.unwrap(), 2);

        let b = encode_chunk(&alph, b"ab", &mut output);
        assert_eq!(output.to_vec(), b"MFRA\0\0\0\0".to_vec());
        assert_eq!(b.unwrap(), 4);

        let b = encode_chunk(&alph, b"abc", &mut output);
        assert_eq!(output.to_vec(), b"MFRGG\0\0\0".to_vec());
        assert_eq!(b.unwrap(), 5);

        let b = encode_chunk(&alph, b"abcd", &mut output);
        assert_eq!(output.to_vec(), b"MFRGGZA\0".to_vec());
        assert_eq!(b.unwrap(), 7);

        let b = encode_chunk(&alph, b"abcde", &mut output);
        assert_eq!(output.to_vec(), b"MFRGGZDF".to_vec());
        assert_eq!(b.unwrap(), 8);

        let b = encode_chunk(&alph, b"", &mut output);
        assert_eq!(b.unwrap(), 0);
    }

    fn encode(data: &[u8], args: &Args) -> String {
        let mut input = Cursor::new(data);
        let output_buf: Vec<u8> = Vec::new();
        let mut output = Cursor::new(output_buf);
        let mut p = Encoder::new(args);
        p.proc(&mut input, &mut output).unwrap();
        String::from_utf8(output.into_inner()).unwrap()
    }

    #[test]
    fn encode_encodes() {
        let res = encode(b"# See more keys and their definitions\n", &Args::default());

        assert_eq!(
            res,
            "EMQFGZLFEBWW64TFEBVWK6LTEBQW4ZBAORUGK2LSEBSGKZTJNZUXI2LPNZZQU===".to_string()
        );
    }

    #[test]
    fn encode_wraps() {
        let res = encode(
            b"# See more keys and their definitions\n",
            &Args {
                wrap: 10,
                ..Default::default()
            },
        );

        assert_eq!(
            res,
            "EMQFGZLFEB\nWW64TFEBVW\nK6LTEBQW4Z\nBAORUGK2LS\nEBSGKZTJNZ\nUXI2LPNZZQ\nU==="
                .to_string()
        );
    }
}
