use super::prepend_linenum::prepend_linenum;
use crate::proc::Processor;

pub struct NumberNonblank {
    linenum: usize,
    last_chunked: bool,
}
impl NumberNonblank {
    pub fn new() -> NumberNonblank {
        NumberNonblank {
            linenum: 0,
            last_chunked: false,
        }
    }
}
impl Processor for NumberNonblank {
    fn proc(&mut self, line: Vec<u8>) -> Option<Vec<u8>> {
        let is_chunked = !line.ends_with(b"\n");

        let retval = if self.last_chunked || line.eq(b"\n") {
            line
        } else {
            self.linenum += 1;
            prepend_linenum(&line, self.linenum)
        };
        self.last_chunked = is_chunked;
        Some(retval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_nonempty_lines_are_numbered() {
        let mut p = NumberNonblank::new();
        let line = b"asd\n".to_vec();
        let empty_line = b"\n".to_vec();
        assert_eq!(p.proc(line.clone()), Some(prepend_linenum(&line, 1)));
        assert_eq!(p.proc(empty_line.clone()), Some(empty_line.clone()));
        assert_eq!(p.proc(line.clone()), Some(prepend_linenum(&line, 2)));
        assert_eq!(p.proc(empty_line.clone()), Some(empty_line.clone()));
    }

    #[test]
    fn chunked_lines_are_not_considered_empty() {
        let mut p = NumberNonblank::new();
        let line = b"asd\n".to_vec();
        let empty_line = b"\n".to_vec();
        let chunked_line = b"asd".to_vec();
        assert_eq!(
            p.proc(chunked_line.clone()),
            Some(prepend_linenum(&chunked_line, 1))
        );
        assert_eq!(p.proc(chunked_line.clone()), Some(chunked_line.clone()));
        assert_eq!(p.proc(empty_line.clone()), Some(empty_line.clone()));
        assert_eq!(p.proc(line.clone()), Some(prepend_linenum(&line, 2)));
        assert_eq!(p.proc(empty_line.clone()), Some(empty_line.clone()));
    }
}
