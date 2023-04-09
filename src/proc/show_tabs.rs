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
                retval.extend("^I".as_bytes());
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
        let line: Vec<u8> = Vec::from("asdf\tdf".as_bytes());
        let mut p = ShowTabs::new();

        assert_eq!(p.proc(line), Some(Vec::from("asdf^Idf".as_bytes())));
    }
}
