use crate::args::Args;
use crate::filtered_reader::FilteredReader;
use crate::proc::Proc;
use base64::{engine::general_purpose, read::DecoderReader};
use std::io::{copy, Read, Write};

pub struct Decoder {
    ignore_garbage: bool,
}

impl Decoder {
    pub fn new(args: &Args) -> Decoder {
        Decoder {
            ignore_garbage: args.ignore_garbage,
        }
    }
}

// As base64 crate alphabet doesn't allow us to get alphabet's bytes, we're getting
// our own version of it.
// TODO investigate sorting it during compile time?..
const ALPHABET: &'static [u8] =
    b"+/0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

impl Proc for Decoder {
    fn proc(&mut self, input: &mut dyn Read, output: &mut dyn Write) -> Result<(), std::io::Error> {
        let mut filtered_reader = FilteredReader::new(
            input,
            match self.ignore_garbage {
                true => |b: u8| matches!(ALPHABET.binary_search(&b), Ok(_)),
                false => |b: u8| !b.is_ascii_whitespace(),
            },
        );
        let mut decoder = DecoderReader::new(&mut filtered_reader, &general_purpose::STANDARD);
        copy(&mut decoder, output)?;
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
    }
}
