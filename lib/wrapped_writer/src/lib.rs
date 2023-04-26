use std::io::Write;

pub struct WrappedWriter<W: Write> {
    output: W,
    wrap: usize,
    remainder: Option<usize>,
    pub bytes_written: usize,
}

impl<W: Write> WrappedWriter<W> {
    pub fn new(output: W, wrap: usize) -> Self {
        Self {
            output: output,
            wrap: wrap,
            remainder: None,
            bytes_written: 0,
        }
    }
    pub fn into_inner(self) -> W {
        self.output
    }
}
impl<W: Write> Write for WrappedWriter<W> {
    /** Write trait specifies, that return value, should be <= bef.len()
     * As we're appending additional symbols to the data, we can't really
     * return number of bytes written, insted we return number of bytes read.
     * But it's still an important information, so we're storing it in the struct
     * so it's accessible for anyone interested.
     */
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.wrap == 0 {
            let n = self.output.write(buf)?;
            self.bytes_written += n;
            return Ok(n);
        }
        self.bytes_written = 0;
        let mut bytes_read_total = 0;
        let chunking_start = match self.remainder {
            Some(remainder) => {
                if remainder > 0 {
                    if remainder >= buf.len() {
                        // If we don't even have enough data to get to next wrapper,
                        // Then we just decrease the remainder and finish the fn
                        let n = self.output.write(buf)?;
                        self.bytes_written += n;
                        bytes_read_total = n;
                        self.remainder = Some(remainder - self.bytes_written);
                        return Ok(bytes_read_total);
                    }
                    bytes_read_total += remainder;
                    self.bytes_written += self.output.write(&buf[..remainder])?;
                }
                self.bytes_written += self.output.write(b"\n")?;
                remainder
            }
            None => 0,
        };

        let mut chunks = buf[chunking_start..].chunks(self.wrap).peekable();
        while let Some(chunk) = chunks.next() {
            // TODO check if n of bytes read is zero?..
            self.bytes_written += self.output.write(chunk)?;
            bytes_read_total += chunk.len();
            if chunks.peek().is_some() {
                self.bytes_written += self.output.write(b"\n")?;
            }
        }
        self.remainder = match buf[chunking_start..].chunks(self.wrap).last() {
            Some(c) if self.wrap > c.len() => Some(self.wrap - c.len()),
            _ => Some(0),
        };
        Ok(bytes_read_total)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.output.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn write(data: &[u8], wrap: usize) -> (Vec<u8>, usize, usize) {
        let mut output_buf: Vec<u8> = Vec::new();
        let cursor = Cursor::new(&mut output_buf);
        let mut writer = WrappedWriter::new(cursor, wrap);
        let n = writer.write(data).unwrap();
        let o = writer.bytes_written;
        return (writer.into_inner().into_inner().to_vec(), n, o);
    }

    #[test]
    fn doesnt_do_anythin_if_wrap_is_zero() {
        let data = b"1234567890";
        let (output, n, o) = write(data, 0);
        assert_eq!(output, data.to_vec());
        assert_eq!(n, data.len());
        assert_eq!(o, data.len());
    }

    #[test]
    fn wraps_with_specified_freq() {
        let data = b"1234567890";
        let expected = b"123\n456\n789\n0".to_vec();
        let (output, n, o) = write(data, 3);
        assert_eq!(output, expected);
        assert_eq!(n, data.len());
        assert_eq!(o, expected.len());
    }

    #[test]
    fn doesnt_wrap_last_chunk() {
        let data = b"123456789";
        let expected = b"123\n456\n789".to_vec();
        let (output, n, o) = write(data, 3);
        assert_eq!(output, expected);
        assert_eq!(n, data.len());
        assert_eq!(o, expected.len());
    }

    #[test]
    fn wraps_can_be_split_between_multiple_writes() {
        let mut output_buf: Vec<u8> = Vec::new();
        let cursor = Cursor::new(&mut output_buf);
        let mut writer = WrappedWriter::new(cursor, 3);
        let n = writer.write(b"12").unwrap();
        assert_eq!(n, 2);
        assert_eq!(writer.bytes_written, 2);
        let n = writer.write(b"3456").unwrap();
        assert_eq!(n, 4);
        assert_eq!(writer.bytes_written, 5); // extra byte because of wrap
        let n = writer.write(b"78").unwrap();
        assert_eq!(n, 2);
        assert_eq!(writer.bytes_written, 3);
        let n = writer.write(b"90").unwrap();
        assert_eq!(n, 2);
        assert_eq!(writer.bytes_written, 3);
        assert_eq!(b"123\n456\n789\n0".to_vec(), output_buf);
    }

    #[test]
    fn input_buffer_can_be_smaller_than_wrap_value() {
        let data = b"123456789";
        let mut output_buf: Vec<u8> = Vec::new();
        let cursor = Cursor::new(&mut output_buf);
        let mut writer = WrappedWriter::new(cursor, 10);
        writer.write(data).unwrap();
        writer.write(data).unwrap();
        assert_eq!(b"1234567891\n23456789".to_vec(), output_buf);
    }
}
