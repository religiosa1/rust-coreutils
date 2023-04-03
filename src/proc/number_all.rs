use crate::proc::Processor;

pub struct NumberAll {
    line: usize,
}
impl NumberAll {
    pub fn new() -> NumberAll {
        NumberAll { line: 0 }
    }
}
impl Processor for NumberAll {
    fn proc(&mut self, line: String) -> Option<String> {
        self.line += 1;
        Some(format!("{: >6} {}", self.line, line))
    }
}
