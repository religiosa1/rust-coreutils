use std::io::Read;

use crate::{entry::Entry, tac_error::TacError};

pub fn split_str<R: Read>(input: &mut R, separator: &str) -> Result<Vec<Entry>, TacError> {
    let separator = separator.as_bytes().to_vec();
    if separator.len() == 0 {
        let mut buf: Vec<u8> = Vec::new();
        input.read_to_end(&mut buf)?;
        return Ok(vec![Entry {
            line: buf,
            separator: separator,
        }]);
    }
    let mut entries: Vec<Entry> = Vec::new();
    let mut n_match = 0;
    let mut line: Vec<u8> = Vec::new();
    for byte in input.bytes() {
        let byte = byte?;
        line.push(byte);
        if byte == separator[n_match] {
            n_match += 1;
            if n_match == separator.len() {
                let sep: Vec<u8> = line.drain(line.len() - n_match..).collect();
                entries.push(Entry {
                    line: line,
                    separator: sep,
                });
                n_match = 0;
                line = Vec::new();
            }
        }
    }
    Ok(entries)
}
