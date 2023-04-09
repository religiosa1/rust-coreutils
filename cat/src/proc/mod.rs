use crate::args;

mod number_all;
mod number_nonblank;
mod prepend_linenum;
mod show_ends;
mod show_nonprinting;
mod show_tabs;
mod squeeze_blank;

pub struct ProcessorDirector {
    processors: Vec<Box<dyn Processor>>,
}

impl ProcessorDirector {
    pub fn new(args: &args::Args) -> ProcessorDirector {
        let mut d = ProcessorDirector {
            processors: Vec::new(),
        };
        if args.squeeze_blank {
            d.processors
                .push(Box::new(squeeze_blank::SqueezeBlank::new()))
        }
        if args.number_nonblank {
            d.processors
                .push(Box::new(number_nonblank::NumberNonblank::new()));
        } else if args.number {
            d.processors.push(Box::new(number_all::NumberAll::new()));
        }
        if args.show_tabs {
            d.processors.push(Box::new(show_tabs::ShowTabs::new()));
        }
        if args.show_ends {
            d.processors.push(Box::new(show_ends::ShowEnds::new()));
        }
        if args.show_nonprinting {
            d.processors
                .push(Box::new(show_nonprinting::ShowNonprinting::new()));
        }
        d
    }

    pub fn proc(&mut self, line: Vec<u8>) -> Option<Vec<u8>> {
        self.processors
            .iter_mut()
            .fold(Some(line), |acc, processor| {
                acc.and_then(|l| processor.proc(l))
            })
    }
}

trait Processor {
    fn proc(&mut self, line: Vec<u8>) -> Option<Vec<u8>>;
}
