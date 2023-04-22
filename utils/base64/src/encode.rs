use crate::args::Args;
use crate::proc::Proc;
use base64::{engine::general_purpose, Engine as _};
use std::io::{Read, Result, Write};

const BUF_SIZE: usize = 8192;
const OUTPUT_BUF_SIZE: usize = BUF_SIZE / 3 * 5;

pub struct Encoder {
    buf: [u8; BUF_SIZE],
    output_buf: [u8; OUTPUT_BUF_SIZE],
    wrap: usize,
}

impl Encoder {
    pub fn new(args: &Args) -> Encoder {
        Encoder {
            buf: [0_u8; BUF_SIZE],
            output_buf: [0_u8; OUTPUT_BUF_SIZE],
            wrap: args.wrap.try_into().unwrap(),
        }
    }
}

impl Proc for Encoder {
    fn proc(&mut self, input: &mut dyn Read, output: &mut dyn Write) -> Result<()> {
        let mut writer = wrapped_writer::WrappedWriter::new(output, self.wrap);

        loop {
            let bytes_read = input.read(&mut self.buf)?;
            if bytes_read == 0 {
                break;
            }
            // TODO Check the padding stuff between two iterations!!!
            let bytes_converted = general_purpose::STANDARD
                .encode_slice(&self.buf[..bytes_read], &mut self.output_buf)
                .unwrap();
            writer.write(&self.output_buf[..bytes_converted])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    fn proc(args: Args, data: &[u8]) -> Vec<u8> {
        let mut output_buf = Vec::new();
        let mut input = Cursor::new(data);
        let mut output = Cursor::new(&mut output_buf);
        let mut p = Encoder::new(&args);
        p.proc(&mut input, &mut output).unwrap();
        output.flush().unwrap();
        output_buf
    }

    #[test]
    fn encodes_some_data() {
        let args = Args {
            wrap: 0,
            ..Default::default()
        };
        let output = proc(args, b"Many hands make light work.");

        assert_eq!(output, b"TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".to_vec());
    }

    #[test]
    fn wraps_data_as_specified() {
        let args = Args {
            wrap: 5,
            ..Default::default()
        };
        let output = proc(args, b"Many hands make light work.");
        assert_eq!(
            output,
            b"TWFue\nSBoYW\n5kcyB\ntYWtl\nIGxpZ\n2h0IH\ndvcms\nu".to_vec()
        );
    }

    #[test]
    fn padding_is_added() {
        let args = Args {
            wrap: 0,
            ..Default::default()
        };
        let output = proc(args, b"light work");
        assert_eq!(output, b"bGlnaHQgd29yaw==".to_vec());
    }
}
