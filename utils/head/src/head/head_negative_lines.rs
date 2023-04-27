use crate::args::Args;
use chunk_reader::ChunkReader;
use std::io::{Read, Write};

use super::head_error::HeadError;

pub fn head_negative_lines<R: Read, W: Write>(
    args: &Args,
    input: R,
    output: &mut W,
) -> Result<(), HeadError> {
    let terminator = if args.zero_terminated { b'\0' } else { b'\n' };
    let lines_to_omit = args.lines.to_usize().ok_or(HeadError::Overflow)?;
    let lines: Vec<Vec<u8>> = input
        .chunks(terminator, 0)
        .collect::<Result<Vec<Vec<u8>>, _>>()?;

    if lines.len() > lines_to_omit {
        for line in &lines[..lines.len() - lines_to_omit] {
            output.write(line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use parse_num::NumValue;

    use super::*;

    const DATA: &'static [u8] = b"1\n\0\n3\n\0\n";

    fn process(data: &[u8], args: &Args) -> Vec<u8> {
        let input = Cursor::new(data);
        let mut output = Cursor::new(Vec::<u8>::new());
        head_negative_lines(&args, input, &mut output).unwrap();
        output.into_inner()
    }

    #[test]
    fn calculates_lines_from_end() {
        let result = process(
            DATA,
            &Args {
                lines: NumValue::from(-2),
                ..Default::default()
            },
        );
        assert_eq!(result, b"1\n\0\n");
    }

    #[test]
    fn calculates_lines_from_end_without_terminator() {
        let result = process(
            &DATA[..DATA.len() - 1],
            &Args {
                lines: NumValue::from(-2),
                ..Default::default()
            },
        );
        assert_eq!(result, b"1\n\0\n");
    }

    #[test]
    fn uses_zero_terminated_lines_if_asked() {
        let result = process(
            &DATA[..DATA.len() - 1],
            &Args {
                lines: NumValue::from(-1),
                zero_terminated: true,
                ..Default::default()
            },
        );
        assert_eq!(result, b"1\n\0");
    }

    #[test]
    fn writes_nothing_if_n_gt_lines_in_file() {
        let result = process(
            &DATA[..DATA.len() - 1],
            &Args {
                lines: NumValue::from(-1000),
                ..Default::default()
            },
        );
        assert_eq!(result, b"");
    }
}
