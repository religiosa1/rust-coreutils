use crate::args::Args;
use crate::proc::Proc;
use base64::{engine::general_purpose, write::EncoderWriter};
use std::io::{copy, Read, Result, Write};

pub struct Encoder {
    wrap: usize,
}

impl Encoder {
    pub fn new(args: &Args) -> Encoder {
        Encoder {
            wrap: args.wrap.try_into().unwrap(),
        }
    }
}

impl Proc for Encoder {
    fn proc(&mut self, input: &mut dyn Read, output: &mut dyn Write) -> Result<()> {
        let wrapped_writer = wrapped_writer::WrappedWriter::new(output, self.wrap);
        let mut writer = EncoderWriter::new(wrapped_writer, &general_purpose::STANDARD);

        copy(input, &mut writer)?;
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
