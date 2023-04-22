use std::io::Write;

pub struct WrappedWriter<W: Write> {
    output: W,
    wrap: usize,
    remainder: Option<usize>,
}

impl<W: Write> WrappedWriter<W> {
    pub fn new(output: W, wrap: usize) -> Self {
        Self {
            output: output,
            wrap: wrap,
            remainder: None,
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.wrap == 0 {
            return self.output.write(buf);
        }
        let mut bytes_written = 0;
        let chunking_start = match self.remainder {
            Some(remainder) => {
                if remainder > 0 {
                    bytes_written += self.output.write(&buf[..remainder])?;
                }
                bytes_written += self.output.write(b"\n")?;
                remainder
            }
            None => 0,
        };

        let mut chunks = buf[chunking_start..].chunks(self.wrap).peekable();
        while let Some(chunk) = chunks.next() {
            bytes_written += self.output.write(chunk)?;
            if chunks.peek().is_some() {
                bytes_written += self.output.write(b"\n")?;
            }
        }
        self.remainder = match buf[chunking_start..].chunks(self.wrap).last() {
            Some(c) if self.wrap > c.len() => Some(self.wrap - c.len()),
            _ => Some(0),
        };
        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn doesnt_do_anythin_if_wrap_is_zero() {
        let data = b"1234567890";
        let mut output_buf: Vec<u8> = Vec::new();
        let cursor = Cursor::new(&mut output_buf);
        let mut writer = WrappedWriter::new(cursor, 0);
        writer.write(data).unwrap();
        assert_eq!(data.to_vec(), output_buf);
    }

    #[test]
    fn wraps_with_specified_freq() {
        let data = b"1234567890";
        let mut output_buf: Vec<u8> = Vec::new();
        let cursor = Cursor::new(&mut output_buf);
        let mut writer = WrappedWriter::new(cursor, 3);
        writer.write(data).unwrap();
        assert_eq!(b"123\n456\n789\n0".to_vec(), output_buf);
    }

    #[test]
    fn doesnt_wrap_last_chunk() {
        let data = b"123456789";
        let mut output_buf: Vec<u8> = Vec::new();
        let cursor = Cursor::new(&mut output_buf);
        let mut writer = WrappedWriter::new(cursor, 3);
        writer.write(data).unwrap();
        assert_eq!(b"123\n456\n789".to_vec(), output_buf);
    }

    #[test]
    fn wraps_can_be_split_between_multiple_writes() {
        let mut output_buf: Vec<u8> = Vec::new();
        let cursor = Cursor::new(&mut output_buf);
        let mut writer = WrappedWriter::new(cursor, 3);
        writer.write(b"12").unwrap();
        writer.write(b"3456").unwrap();
        writer.write(b"78").unwrap();
        writer.write(b"90").unwrap();
        assert_eq!(b"123\n456\n789\n0".to_vec(), output_buf);
    }
}
