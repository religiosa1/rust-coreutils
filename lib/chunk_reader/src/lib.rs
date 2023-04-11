use std::io;
use std::io::Read;

/** Reading a chunk from a reader, until b'\n' is encountered or until chunk_size is met
 *
 * Basically, the same as split() with a limit.
 *
 * Notice, that every byte is read with a separate call to read() function, so to avoid
 * excessive system calls when reading from for example a file, don't forget to use BufReader
 */

pub struct Chunked<B> {
    inner: B,
    delim: u8,
    chunk_size: usize,
    done: bool,
}

impl<B: Read> Iterator for Chunked<B> {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<io::Result<Vec<u8>>> {
        if self.done {
            return None;
        }
        let capacity = if self.chunk_size > 0 {
            self.chunk_size
        } else {
            0
        };
        let mut retval: Vec<u8> = Vec::with_capacity(capacity);

        loop {
            let mut buf = [0; 1];
            match self.inner.read(&mut buf) {
                Ok(0) => {
                    self.done = true;
                    if retval.len() > 0 {
                        return Some(Ok(retval));
                    } else {
                        return None;
                    }
                }
                Ok(_) => {
                    let b = buf[0];
                    retval.push(b);
                    if b == self.delim || (self.chunk_size > 0 && retval.len() >= self.chunk_size) {
                        return Some(Ok(retval));
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
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
            inner: self,
            delim: delim,
            chunk_size: chunk_size,
            done: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn returns_vecs_split_by_separator() {
        let data = b"123b456bb";
        let cursor = Cursor::new(data);

        let result: io::Result<Vec<Vec<u8>>> = cursor.chunks(b'b', 5).collect();
        let matrix = result.unwrap();

        assert_eq!(
            matrix,
            vec![b"123b".to_vec(), b"456b".to_vec(), b"b".to_vec()]
        );
    }

    #[test]
    fn returns_chunk_if_chunk_size_achieved() {
        let data = b"123b456789bb";
        let cursor = Cursor::new(data);

        let result: io::Result<Vec<Vec<u8>>> = cursor.chunks(b'b', 5).collect();
        let matrix = result.unwrap();

        assert_eq!(
            matrix,
            vec![
                b"123b".to_vec(),
                b"45678".to_vec(),
                b"9b".to_vec(),
                b"b".to_vec()
            ]
        );
    }

    #[test]
    fn zero_chunk_size_means_unlimited_chunk() {
        let data = b"123b456789bb";
        let cursor = Cursor::new(data);

        let result: io::Result<Vec<Vec<u8>>> = cursor.chunks(b'b', 0).collect();
        let matrix = result.unwrap();

        assert_eq!(
            matrix,
            vec![b"123b".to_vec(), b"456789b".to_vec(), b"b".to_vec()]
        );
    }
}
