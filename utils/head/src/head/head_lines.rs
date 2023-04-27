use crate::args::Args;
use chunk_reader::ChunkReader;
use ibig::UBig;
use std::io::{Read, Write};

use super::head_error::HeadError;

const CHUNK_SIZE: usize = 8192;

pub fn head_lines<R: Read, W: Write>(
    args: &Args,
    input: R,
    output: &mut W,
) -> Result<(), HeadError> {
    let terminator = if args.zero_terminated { b'\0' } else { b'\n' };
    let mut counter = UBig::from(0_usize);
    let max = args.lines.to_ubig();
    for chunk in input.chunks(terminator, CHUNK_SIZE) {
        if counter >= max {
            break;
        }
        let chunk = chunk?;
        output.write(&chunk)?;

        if chunk.ends_with(&[terminator]) {
            counter += 1;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn takes_the_specified_amount_of_lines() {
        let input = Cursor::new(b"asd\nb\0c\nq\0e");
        let mut output = Cursor::new(Vec::<u8>::new());
        let args = Args {
            lines: 2.into(),
            ..Default::default()
        };
        head_lines(&args, input, &mut output).unwrap();
        assert_eq!(output.into_inner(), b"asd\nb\0c\n");
    }

    #[test]
    fn splits_by_zero_if_asked() {
        let input = Cursor::new(b"asd\nb\0c\nq\0e");
        let mut output = Cursor::new(Vec::<u8>::new());
        let args = Args {
            lines: 2.into(),
            zero_terminated: true,
            ..Default::default()
        };
        head_lines(&args, input, &mut output).unwrap();
        assert_eq!(output.into_inner(), b"asd\nb\0c\nq\0");
    }

    #[test]
    fn handles_longish_input() {
        let input = Cursor::new([b"a".repeat(CHUNK_SIZE + 100), b"\nasdf".to_vec()].concat());
        let mut output = Cursor::new(Vec::<u8>::new());
        let args = Args {
            lines: 1.into(),
            ..Default::default()
        };
        head_lines(&args, input, &mut output).unwrap();
        assert_eq!(
            output.into_inner(),
            [b"a".repeat(CHUNK_SIZE + 100), b"\n".to_vec()].concat()
        );
    }
}
