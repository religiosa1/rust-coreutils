use crate::proc::Processor;

pub struct ShowEnds;
impl ShowEnds {
    pub fn new() -> ShowEnds {
        ShowEnds
    }
}
impl Processor for ShowEnds {
    fn proc(&mut self, line: String) -> Option<String> {
        Some(format!("{}$", line))
    }
}
