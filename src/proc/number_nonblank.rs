use crate::proc::Processor;

pub struct NumberNonblank {
    line: usize,
}
impl NumberNonblank {
    pub fn new() -> NumberNonblank {
        NumberNonblank { line: 0 }
    }
}
impl Processor for NumberNonblank {
    fn proc(&mut self, line: String) -> Option<String> {
        if line != "" {
            self.line += 1;
            Some(format!("{: >6} {}", self.line, line))
        } else {
            Some(line)
        }
    }
}
