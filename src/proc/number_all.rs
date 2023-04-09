use super::prepend_linenum::prepend_linenum;
use crate::proc::Processor;

pub struct NumberAll {
    linenum: usize,
    last_chunked: bool,
}
impl NumberAll {
    pub fn new() -> NumberAll {
        NumberAll {
            linenum: 0,
            last_chunked: false,
        }
    }
}
impl Processor for NumberAll {
    fn proc(&mut self, line: Vec<u8>) -> Option<Vec<u8>> {
        let is_chunked = !line.ends_with(&[b'\n']);
        let retval = if self.last_chunked {
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
    fn adds_a_number_to_a_line() {
        let line1: Vec<u8> = vec![b'a', b'b', b'c', b'\n'];
        let line2: Vec<u8> = vec![b'e', b'f'];

        let mut p = NumberAll::new();

        assert_eq!(p.proc(line1.clone()), Some(prepend_linenum(&line1, 1)));
        assert_eq!(p.proc(line2.clone()), Some(prepend_linenum(&line2, 2)));
    }

    #[test]
    fn if_the_line_was_chuncked_doesnt_add_a_number() {
        let line1: Vec<u8> = vec![b'a', b'b', b'c'];
        let line2: Vec<u8> = vec![b'e', b'f'];

        let mut p = NumberAll::new();

        assert_eq!(p.proc(line1.clone()), Some(prepend_linenum(&line1, 1)));
        assert_eq!(p.proc(line2.clone()), Some(line2));
    }
}
