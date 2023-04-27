use super::head_error::HeadError;
use crate::args::Args;
use std::io::{Read, Write};

pub fn head_negative_bytes<R: Read, W: Write>(
    args: &Args,
    mut input: R,
    output: &mut W,
) -> Result<(), HeadError> {
    let bytes_to_omit = args
        .bytes
        .as_ref()
        .unwrap()
        .to_usize()
        .ok_or(HeadError::Overflow)?;
    let mut buffer: Vec<u8> = Vec::new();
    input.read_to_end(&mut buffer)?;
    if bytes_to_omit < buffer.len() {
        output.write(&buffer[..buffer.len() - bytes_to_omit])?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use parse_num::NumValue;

    use super::*;

    #[test]
    fn calculate_bytes_from_end() {
        let input = Cursor::new(b"1234567890");
        let mut output = Cursor::new(Vec::<u8>::new());
        let args = Args {
            bytes: Some(NumValue::from(-2)),
            ..Default::default()
        };
        head_negative_bytes(&args, input, &mut output).unwrap();
        assert_eq!(output.into_inner(), b"12345678");
    }

    #[test]
    fn writes_nothing_if_n_gt_input_length() {
        let input = Cursor::new(b"1234567890");
        let mut output = Cursor::new(Vec::<u8>::new());
        let args = Args {
            bytes: Some(NumValue::from(-15)),
            ..Default::default()
        };
        head_negative_bytes(&args, input, &mut output).unwrap();
        assert_eq!(output.into_inner(), b"");
    }
}
