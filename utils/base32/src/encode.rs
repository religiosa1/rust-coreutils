use crate::alphabet::{Alphabet, RFC4648_ALPHABET};
use crate::{args::Args, proc::Proc};
use std::io::{Read, Result, Write};

const BUF_SIZE: usize = 8192;

pub struct Encoder {
    buf: [u8; BUF_SIZE],
    output_buf: Vec<u8>,
    wrap: usize,
}

impl Encoder {
    pub fn new(args: &Args) -> Encoder {
        Encoder {
            buf: [0_u8; BUF_SIZE],
            output_buf: Vec::with_capacity(BUF_SIZE * 2),
            wrap: args.wrap.try_into().unwrap(),
        }
    }
}

impl Proc for Encoder {
    fn proc(&mut self, input: &mut dyn Read, output: &mut dyn Write) -> Result<()> {
        let mut writer = wrapped_writer::WrappedWriter::new(output, self.wrap);
        loop {
            let bytes_read = input.read(&mut self.buf)?;
            let mut bytes_converted = 0_usize;
            if bytes_read == 0 {
                break;
            }
            for chunk in self.buf.chunks(5) {
                // TODO check that chunks has 5 bytes and it's not the end of the file
                // (so we add padding only when it's needed)
                let n_bytes = encode_chunk(
                    &RFC4648_ALPHABET,
                    chunk,
                    &mut self.output_buf[bytes_converted..],
                );
                bytes_converted += n_bytes;
            }
            writer.write(&self.output_buf[..bytes_converted])?;
        }
        Ok(())
    }
}

fn encode_chunk(alphabet: &Alphabet, chunk: &[u8], output: &mut [u8]) -> usize {
    let b0 = match chunk.get(0).copied() {
        Some(b) => b,
        None => return 0,
    };
    let b1 = chunk.get(1).copied();
    let b2 = chunk.get(2).copied();
    let b3 = chunk.get(3).copied();
    let b4 = chunk.get(4).copied();

    output[0] = alphabet.symbol((b0 & 0b1111_1000) >> 3);
    output[1] = alphabet.symbol((b0 & 0b111) << 5 | (b1.unwrap_or(0_u8) & 0b1100_0000) >> 6);
    let mut bytes_written = 2;

    if let Some(b) = b1 {
        output[2] = (b & 0b0011_1110) >> 1;
        output[3] = (b & 0b1) << 7 | (b2.unwrap_or(0) & 0b1111_0000) >> 4;
        bytes_written = 4;
    }
    if let Some(b) = b2 {
        output[4] = (b & 0b1111) << 4 & (b3.unwrap_or(0) & 0b1000_0000) >> 7;
        bytes_written = 5;
    }
    if let Some(b) = b3 {
        output[5] = (b & 0b0111_1100) >> 2;
        output[6] = (b & 0b11) << 6 | (b4.unwrap_or(0) & 0b1110_0000) >> 5;
        bytes_written = 7;
    }
    if let Some(b) = b4 {
        output[7] = b & 0b1_1111;
        bytes_written = 8;
    }

    if bytes_written < 8 {
        if let Some(p) = alphabet.padding {
            output[bytes_written..8].fill(p);
            bytes_written = 8;
        }
    }

    return bytes_written;
}
