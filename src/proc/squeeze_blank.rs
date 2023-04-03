use crate::proc::Processor;

pub struct SqueezeBlank {
    last_blank: bool,
}
impl SqueezeBlank {
    pub fn new() -> SqueezeBlank {
        SqueezeBlank { last_blank: false }
    }
}
impl Processor for SqueezeBlank {
    fn proc(&mut self, line: String) -> Option<String> {
        if line == "" {
            let retval = if self.last_blank {
                None
            } else {
                Some(String::from(""))
            };
            self.last_blank = true;
            retval
        } else {
            self.last_blank = false;
            Some(line)
        }
    }
}
