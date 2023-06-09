use crate::proc::Processor;

pub struct ShowEnds;
impl ShowEnds {
    pub fn new() -> ShowEnds {
        ShowEnds
    }
}
impl Processor for ShowEnds {
    fn proc(&mut self, line: Vec<u8>) -> Option<Vec<u8>> {
        let mut retval: Vec<u8> = Vec::new();
        for c in line {
            if c == b'\n' {
                retval.extend(b"$\n");
            } else {
                retval.push(c);
            }
        }
        Some(retval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_usd_to_line_ends() {
        let line: Vec<u8> = b"asdf\ndf".to_vec();
        let mut p = ShowEnds::new();

        assert_eq!(p.proc(line), Some(b"asdf$\ndf".to_vec()));
    }
}
