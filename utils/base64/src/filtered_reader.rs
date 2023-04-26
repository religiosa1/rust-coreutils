use std::io::Read;

pub struct FilteredReader<R: Read> {
    inner: R,
    filter_fn: Box<dyn Fn(u8) -> bool>,
    bytes_read: usize,
}

impl<R: Read> FilteredReader<R> {
    pub fn new(inner: R, filter_fn: impl Fn(u8) -> bool + 'static) -> Self {
        Self {
            inner: inner,
            filter_fn: Box::new(filter_fn),
            bytes_read: 0_usize,
        }
    }
}

impl<R: Read> Read for FilteredReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let mut temp = [0_u8; 1];
        self.bytes_read = 0;
        let mut total_bytes_written = 0_usize;
        while total_bytes_written < buf.len() {
            let n = self.inner.read(&mut temp)?;
            self.bytes_read += n;
            if n == 0 {
                break;
            }
            if (self.filter_fn)(temp[0]) {
                buf[total_bytes_written] = temp[0];
                total_bytes_written += 1;
            }
        }
        Ok(total_bytes_written)
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn filters_out_data() {
        let data = b"a1b2c3";
        let cursor = Cursor::new(data);
        let mut reader = FilteredReader::new(cursor, |b: u8| b.is_ascii_digit());
        let mut buf = [0_u8; 5];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(buf.to_vec(), b"123\0\0");
        assert_eq!(n, 3);
        assert_eq!(reader.bytes_read, data.len());
    }

    #[test]
    fn doesnt_read_above_buffer_capacity() {
        let data = b"a1b2c3";
        let cursor = Cursor::new(data);
        let mut reader = FilteredReader::new(cursor, |b: u8| b.is_ascii_digit());
        let mut buf = [0_u8; 2];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(buf.to_vec(), b"12");
        assert_eq!(n, 2);
        assert_eq!(reader.bytes_read, 4);
    }
}
