use super::tail_error::TailError;
use crate::args::Args;
use std::collections::VecDeque;
use std::io::{BufRead, Write};

pub fn tail_bytes<R: BufRead>(args: &Args, input: R) -> Result<(), TailError> {
    let bytes = get_bytes(&args, input)?;
    let (slice1, slice2) = bytes.as_slices();
    std::io::stdout().write(slice1)?;
    std::io::stdout().write(slice2)?;
    std::io::stdout().flush()?;
    Ok(())
}

pub fn get_bytes<R: BufRead>(args: &Args, input: R) -> Result<VecDeque<u8>, TailError> {
    let n = args
        .bytes
        .as_ref()
        .unwrap()
        .to_usize()
        .ok_or(TailError::Overflow)?;
    let mut bytes: VecDeque<u8> = VecDeque::with_capacity(n);
    for byte in input.bytes() {
        let byte = byte?;
        bytes.push_back(byte);
        if bytes.len() > n {
            bytes.pop_front();
        }
    }
    Ok(bytes)
}

#[cfg(test)]
mod test {
    use parse_num::NumValue;

    use super::*;
    use std::io::{BufReader, Cursor};

    #[test]
    fn reads_the_specified_amount_of_bytes_from_the_end() {
        let data = b"123456789";
        let cursor = Cursor::new(data);
        let input = BufReader::new(cursor);
        let args = Args {
            bytes: Some(NumValue {
                prefix: None,
                value: 2,
                multiplier: None,
            }),
            ..Args::default()
        };
        let result = get_bytes(&args, input).unwrap();
        assert_eq!(result, VecDeque::from(b"89".to_vec()))
    }
}
