use crate::proc::Processor;

pub struct SqueezeBlank {
    last_blank: bool,
    last_chunked: bool,
}
impl SqueezeBlank {
    pub fn new() -> SqueezeBlank {
        SqueezeBlank {
            last_blank: false,
            last_chunked: false,
        }
    }
}
impl Processor for SqueezeBlank {
    fn proc(&mut self, line: Vec<u8>) -> Option<Vec<u8>> {
        let is_chunked = !line.ends_with(b"\n");

        let retval = if self.last_chunked {
            self.last_blank = line.len() == 0;
            Some(line)
        } else {
            let is_blank = line.eq(b"\n");
            let last_blank = self.last_blank;
            self.last_blank = is_blank;
            if is_blank && last_blank {
                None
            } else {
                Some(line)
            }
        };
        self.last_chunked = is_chunked;
        retval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_blank_lines_are_ommited() {
        let mut p = SqueezeBlank::new();
        let line = b"asd\n".to_vec();
        let empty_line = b"\n".to_vec();

        assert_eq!(p.proc(line.clone()), Some(line.clone()));
        assert_eq!(p.proc(empty_line.clone()), Some(empty_line.clone()));
        assert_eq!(p.proc(empty_line.clone()), None);
        assert_eq!(p.proc(empty_line.clone()), None);
        assert_eq!(p.proc(line.clone()), Some(line.clone()));
    }

    #[test]
    fn chunked_lines_are_not_considered_as_blanks() {
        let mut p = SqueezeBlank::new();
        let line = b"asd\n".to_vec();
        let chunked_line = b"asd".to_vec();
        let empty_line = b"\n".to_vec();

        assert_eq!(p.proc(chunked_line.clone()), Some(chunked_line.clone()));
        assert_eq!(p.proc(chunked_line.clone()), Some(chunked_line.clone()));
        assert_eq!(p.proc(empty_line.clone()), Some(empty_line.clone()));
        assert_eq!(p.proc(empty_line.clone()), Some(empty_line.clone()));
        assert_eq!(p.proc(empty_line.clone()), None);
        assert_eq!(p.proc(line.clone()), Some(line.clone()));
    }
}
