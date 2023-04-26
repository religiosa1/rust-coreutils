use std::io::Read;

use regex::bytes::Regex;

use crate::{entry::Entry, tac_error::TacError};

pub fn split_regex<R: Read>(input: &mut R, regex: &str) -> Result<Vec<Entry>, TacError> {
    let re = Regex::new(format!(r"(?P<line>.+?)(?:(?P<sep>{})|$)", regex).as_str())?;
    let mut data: Vec<u8> = Vec::new();
    input.read_to_end(&mut data)?;
    let entries: Vec<Entry> = re
        .captures_iter(&data)
        .map(|caps| Entry {
            line: caps["line"].to_vec(),
            separator: match caps.name("sep") {
                Some(m) => m.as_bytes().to_vec(),
                _ => b"".to_vec(),
            },
        })
        .collect();
    Ok(entries)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn splits_the_file() {
        let mut input = Cursor::new(b"asd\nqew\t\txcv\tsdf");
        let entries = split_regex(&mut input, r"\s+").unwrap();
        assert_eq!(
            entries,
            vec![
                Entry {
                    line: b"asd".to_vec(),
                    separator: b"\n".to_vec(),
                },
                Entry {
                    line: b"qew".to_vec(),
                    separator: b"\t\t".to_vec(),
                },
                Entry {
                    line: b"xcv".to_vec(),
                    separator: b"\t".to_vec(),
                },
                Entry {
                    line: b"sdf".to_vec(),
                    separator: b"".to_vec(),
                },
            ]
        );
    }
}
