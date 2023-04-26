use clap::Parser;

// @see https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

/// output the last part of files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub file: Vec<String>,

    /// attach the separator before instead of after
    #[arg(short = 'b', long, default_value_t = false)]
    pub before: bool,

    /// interpret the separator as a regular expression
    #[arg(short = 'r', long, default_value_t = false)]
    pub regex: bool,

    /// use STRING as the separator instead of newline
    #[arg(short = 's', long, default_value_t = String::from("\n"))]
    pub separator: String,
}

impl Args {
    pub fn parse() -> Args {
        let mut args = <Self as Parser>::parse();
        if args.file.len() == 0 {
            args.file.push("-".into());
        }
        args
    }
}
