use chunk_reader::ChunkReader;

use super::tail_error::TailError;
use crate::args::Args;
use std::collections::VecDeque;
use std::io::{Read, Write};

pub fn tail_lines<R: Read>(args: &Args, input: R) -> Result<(), TailError> {
    let lines = get_lines(args, input)?;
    for line in lines {
        std::io::stdout().write(&line)?;
    }
    std::io::stdout().flush()?;
    Ok(())
}

fn get_lines<R: Read>(args: &Args, input: R) -> Result<VecDeque<Vec<u8>>, TailError> {
    let n = args.lines.to_usize().ok_or(TailError::Overflow)?;
    let mut q: VecDeque<Vec<u8>> = VecDeque::with_capacity(n);

    for line in input.chunks(args.terminator, 0) {
        let line = line?;
        q.push_back(line);
        if q.len() > n {
            q.pop_front();
        }
    }
    Ok(q)
}

#[cfg(test)]
mod test {
    use parse_num::NumValue;

    use super::*;
    use std::io::Cursor;

    #[test]
    fn reads_the_required_number_of_lines() {
        let data = b"1\n2\n3\n4\n5\n";
        let cursor = Cursor::new(data);
        let args = Args {
            terminator: b'\n',
            lines: NumValue {
                prefix: None,
                value: 2,
                multiplier: None,
            },
            ..Args::default()
        };
        let result = get_lines(&args, cursor).unwrap();
        assert_eq!(result, VecDeque::from([b"4\n".to_vec(), b"5\n".to_vec()]));
    }

    #[test]
    fn outputs_all_if_lines_gt_file() {
        let data = b"1\n2\n3\n4\n5\n";
        let cursor = Cursor::new(data);
        let args = Args {
            terminator: b'\n',
            lines: NumValue {
                prefix: None,
                value: 10,
                multiplier: None,
            },
            ..Args::default()
        };
        let result = get_lines(&args, cursor).unwrap();
        assert_eq!(
            result,
            VecDeque::from([
                b"1\n".to_vec(),
                b"2\n".to_vec(),
                b"3\n".to_vec(),
                b"4\n".to_vec(),
                b"5\n".to_vec()
            ])
        );
    }
}
