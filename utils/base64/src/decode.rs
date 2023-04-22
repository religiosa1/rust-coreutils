use crate::args::Args;
use crate::proc::Proc;
use base64::{engine::general_purpose, Engine as _};
use std::io::{Read, Write};

const BUF_SIZE: usize = 8192;
const OUTPUT_BUF_SIZE: usize = BUF_SIZE / 4 * 3;

pub struct Decoder {
    buf: [u8; BUF_SIZE],
    filtered_buf: [u8; BUF_SIZE],
    output_buf: [u8; OUTPUT_BUF_SIZE],
    ignore_garbage: bool,
}

impl Decoder {
    pub fn new(args: &Args) -> Decoder {
        Decoder {
            buf: [0_u8; BUF_SIZE],
            filtered_buf: [0_u8; BUF_SIZE],
            output_buf: [0_u8; OUTPUT_BUF_SIZE],
            ignore_garbage: args.ignore_garbage,
        }
    }
}

impl Proc for Decoder {
    fn proc(&mut self, input: &mut dyn Read, output: &mut dyn Write) -> Result<(), std::io::Error> {
        loop {
            let bytes_read = input.read(&mut self.buf)?;
            if bytes_read == 0 {
                break;
            }
            let bytes_filtered = filter_bad_bytes(
                &self.buf[..bytes_read],
                &mut self.filtered_buf,
                self.ignore_garbage,
            );
            let bytes_converted = general_purpose::STANDARD
                .decode_slice(&self.filtered_buf[..bytes_filtered], &mut self.output_buf)
                .unwrap();
            output.write(&self.output_buf[..bytes_converted])?;
        }
        Ok(())
    }
}

// As base64 crate alphabet doesn't allow us to get alphabet's bytes, we're getting
// our own version of it.
// TODO investigate sorting it during compile time?..
const ALPHABET: &'static [u8] =
    b"+/0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn filter_bad_bytes(input: &[u8], output: &mut [u8], ignore_garbage: bool) -> usize {
    let mut counter = 0;
    let filter_fn = match ignore_garbage {
        true => |byte: &u8| matches!(ALPHABET.binary_search(byte), Ok(_)),
        false => |byte: &u8| !byte.is_ascii_whitespace(),
    };
    for byte in input {
        if filter_fn(byte) {
            output[counter] = *byte;
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn filter_bad_bytes_filters() {
        let mut buf = [0_u8; 3];
        let filtered = filter_bad_bytes(b"a$ b\n^c\t", &mut buf, true);
        assert_eq!(filtered, 3);
        assert_eq!(buf.to_vec(), b"abc".to_vec());
    }

    #[test]
    fn filter_bad_bytes_whitespace() {
        let mut buf = [0_u8; 5];
        let filtered = filter_bad_bytes(b"a$ b\n^c\t", &mut buf, false);
        assert_eq!(filtered, 5);
        assert_eq!(buf.to_vec(), b"a$b^c".to_vec());
    }

    fn proc(args: Args, data: &[u8]) -> Vec<u8> {
        let mut output_buf = Vec::new();
        let mut input = Cursor::new(data);
        let mut output = Cursor::new(&mut output_buf);
        let mut p = Decoder::new(&args);
        p.proc(&mut input, &mut output).unwrap();
        output.flush().unwrap();
        output_buf
    }

    #[test]
    fn decodes_some_data() {
        let output = proc(Args::default(), b"TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");

        assert_eq!(output, b"Many hands make light work.".to_vec());
    }

    #[test]
    fn ignores_whitespaces() {
        let output = proc(
            Args::default(),
            b"TWFue\nSBoYW\n  5kcyB\ntYWtl\nIGxpZ\n2h0IH\ndvcms\nu",
        );
        assert_eq!(output, b"Many hands make light work.".to_vec());
        // TODO true negative test
    }

    #[test]
    fn ignores_garbage_if_asked() {
        let args = Args {
            ignore_garbage: true,
            ..Args::default()
        };
        let output = proc(
            args,
            b"TWF$$$ue\nSBoYW\n  5kcyB^^^\ntYWtl\nIGxpZ\n2h0IH\ndvcms\nu",
        );
        assert_eq!(output, b"Many hands make light work.".to_vec());
        // TODO true negative test
    }
}
