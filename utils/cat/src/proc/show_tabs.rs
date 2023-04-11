use crate::proc::Processor;

pub struct ShowTabs;
impl ShowTabs {
    pub fn new() -> ShowTabs {
        ShowTabs
    }
}
impl Processor for ShowTabs {
    fn proc(&mut self, line: Vec<u8>) -> Option<Vec<u8>> {
        let mut retval: Vec<u8> = Vec::new();
        for c in line {
            if c == b'\t' {
                retval.extend(b"^I");
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
    fn replaces_tabs_with_i() {
        let line: Vec<u8> = b"asdf\tdf".to_vec();
        let mut p = ShowTabs::new();

        assert_eq!(p.proc(line), Some(b"asdf^Idf".to_vec()));
    }
}
