/* Reading a chunk from a reader, until b'\n' is encountered or until CHUNK_SIZE is met */
use std::io::{BufReader, Read, Result};

pub struct Chunked<B> {
    buf: B,
    delim: u8,
    chunk_size: usize,
    done: bool,
}

impl<B: Read> Iterator for Chunked<B> {
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Result<Vec<u8>>> {
        if self.done {
            return None;
        }
        let mut retval: Vec<u8> = Vec::new();
        let reader = BufReader::new(&mut self.buf);

        for byte in reader.bytes().take(self.chunk_size) {
            match byte {
                Ok(0) => {
                    self.done = true;
                    break;
                }
                Ok(b) => {
                    retval.push(b);
                    if b == self.delim {
                        break;
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
        Some(Ok(retval))
    }
}

pub trait ChunkReader: Read {
    fn chunks(self, delim: u8, chunk_size: usize) -> Chunked<Self>
    where
        Self: Sized;
}

impl<T: Read> ChunkReader for T {
    fn chunks(self, delim: u8, chunk_size: usize) -> Chunked<T> {
        Chunked {
            buf: self,
            delim: delim,
            chunk_size: chunk_size,
            done: false,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn returns_vecs_split_by_separator() {}

//     #[test]
//     fn returns_chunk_if_chunk_size_achieved() {}
// }
