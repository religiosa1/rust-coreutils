use crate::proc::Processor;

pub struct ShowTabs;
impl ShowTabs {
    pub fn new() -> ShowTabs {
        ShowTabs
    }
}
impl Processor for ShowTabs {
    fn proc(&mut self, line: String) -> Option<String> {
        Some(line.replace("\t", "^I"))
    }
}
