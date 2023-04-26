use clap::Parser;

// @see https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

/// base32 encode/decode data and print to standard output
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub file: Vec<String>,

    /// decode data
    #[arg(short = 'd', long, default_value_t = false)]
    pub decode: bool,

    /// when decoding, ignore non-alphabet characters
    #[arg(short = 'i', long, default_value_t = false)]
    pub ignore_garbage: bool,

    /// wrap encoded lines after COLS character.  Use 0 to disable line wrapping
    #[arg(short = 'w', long, default_value_t = 76)]
    pub wrap: u32,
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
