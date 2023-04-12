use clap::{Parser, ValueEnum};
use parse_num::{parse_num, NumValue};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FollowMode {
    /// Follow the file by it's name
    Name,
    /// Follow the file by it's fd (continue tracking after rename)
    Descriptor,
}

// @see https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

/// output the last part of files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub file: Vec<String>,

    /// output the last NUM bytes; or use -c +NUM to output starting with byte NUM of each file
    #[arg(short = 'c', long, value_parser=parse_num)]
    pub bytes: Option<NumValue>,

    /// output appended data as the file grows
    #[arg(short = 'f', long, value_enum)]
    pub follow: Option<FollowMode>,

    /// same as --follow=name --retry
    #[arg(short = 'F', default_value_t = false)]
    pub fr: bool,

    /// output the last NUM lines, instead of the last 10; or use -n +NUM to output starting with line NUM
    #[arg(short = 'n', long, value_parser=parse_num, default_value = "10")]
    pub lines: NumValue,

    /// with -f, terminate after process ID, PID dies
    #[arg(long, default_value_t = true)]
    pub pid: bool,

    /// never print headers giving file names
    #[arg(short = 'q', long, visible_alias = "silent", default_value_t = false)]
    pub quiet: bool,

    /// keep trying to open a file if it is inaccessible
    #[arg(long, default_value_t = false)]
    pub retry: bool,

    /// with --pid=P check process P at least once every N seconds
    #[arg(short = 's', long, default_value_t = 1.0, requires = "pid")]
    pub sleep_interval: f32,

    /// always print headers giving file names
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// line delimiter is NUL, not newline
    #[arg(short = 'z', long, default_value_t = false)]
    pub zero_terminated: bool,

    // computed based on other args
    pub print_headers: bool,
    pub terminator: u8,
}

impl Args {
    pub fn parse() -> Args {
        let mut args = <Self as Parser>::parse();
        if args.file.len() == 0 {
            args.file.push("-".into());
        }
        if args.fr {
            args.retry = true;
            args.follow = Some(FollowMode::Name);
        }
        args.print_headers = (args.file.len() > 1 || args.verbose) && !args.quiet;
        args.terminator = if args.zero_terminated { b'\0' } else { b'\n' };
        args
    }
}
