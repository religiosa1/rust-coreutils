use clap::Parser;

// @see https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

/// concatenate files and print on the standard output
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub file: Vec<String>,

    /// equivalent to -vET
    #[arg(short = 'A', long, default_value_t = false)]
    pub show_all: bool,

    /// number nonempty output lines, overrides -n
    #[arg(short = 'b', long, default_value_t = false)]
    pub number_nonblank: bool,

    // equivalent to -vE
    #[arg(short = 'e', default_value_t = false)]
    pub ve: bool,

    /// display $ at end of each line
    #[arg(short = 'E', long, default_value_t = false)]
    pub show_ends: bool,

    /// number all output lines
    #[arg(short = 'n', long, default_value_t = false)]
    pub number: bool,

    /// suppress repeated empty output lines
    #[arg(short = 's', long, default_value_t = false)]
    pub squeeze_blank: bool,

    /// equivalent to -vT
    #[arg(short = 't', default_value_t = false)]
    pub vt: bool,

    /// display TAB characters as ^I
    #[arg(short = 'T', long, default_value_t = false)]
    pub show_tabs: bool,

    /// (ignored)
    #[arg(short = 'u', default_value_t = false)]
    pub ignored: bool,

    /// use ^ and M- notation, except for LFD and TAB
    #[arg(short = 'v', long, default_value_t = false)]
    pub show_nonprinting: bool,
}

impl Args {
    // We're overriding and extending parser method, as we need to expand
    // "equivalent" flags, that turn on multiple args at once.
    pub fn parse() -> Args {
        let mut args = <Self as Parser>::parse();
        if args.show_all {
            args.show_tabs = true;
            args.show_ends = true;
            args.show_nonprinting = true;
        } else if args.ve {
            args.show_ends = true;
            args.show_nonprinting = true;
        }
        if args.vt {
            args.show_tabs = true;
            args.show_nonprinting = true;
        }
        if args.file.len() == 0 {
            args.file.push("-".into());
        }
        args
    }
}
