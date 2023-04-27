use crate::args::Args;
use ibig::{ops::DivRem, UBig};
use std::io::{Read, Write};

const CHUNK_SIZE: usize = 8192;

use super::head_error::HeadError;

pub fn head_bytes<R: Read, W: Write>(
    args: &Args,
    mut input: R,
    output: &mut W,
) -> Result<(), HeadError> {
    let mut buffer = [0_u8; CHUNK_SIZE];
    let max = args.bytes.clone().unwrap().to_ubig();
    let (n_full_chunks, partial_read) = max.div_rem(CHUNK_SIZE);
    let mut counter = UBig::from(0_usize);

    if n_full_chunks > UBig::from(0_usize) {
        while counter < n_full_chunks {
            let bytes_read = input.read(&mut buffer)?;
            counter += 1;
            if bytes_read == 0 {
                return Ok(());
            }
            output.write(&buffer[..bytes_read])?;
        }
    }
    if partial_read > 0 {
        let bytes_read = input.read(&mut buffer[..partial_read])?;
        if bytes_read == 0 {
            return Ok(());
        }
        output.write(&buffer[..bytes_read])?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn takes_the_specified_amount_of_bytes() {
        let input = Cursor::new(b"0123456789");
        let mut output = Cursor::new(Vec::<u8>::new());
        let args = Args {
            bytes: Some(5.into()),
            ..Default::default()
        };
        head_bytes(&args, input, &mut output).unwrap();
        assert_eq!(output.into_inner(), b"01234");
    }

    #[test]
    fn partial_and_long_buffer_read_as_expected() {
        let input = Cursor::new(b"x".repeat(CHUNK_SIZE + 200));
        let take = CHUNK_SIZE + 100;
        let mut output = Cursor::new(Vec::<u8>::new());
        let args = Args {
            bytes: Some(take.into()),
            ..Default::default()
        };
        head_bytes(&args, input, &mut output).unwrap();
        assert_eq!(output.into_inner(), b"x".repeat(take));
    }
}
