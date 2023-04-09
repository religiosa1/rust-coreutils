use clap::Parser;

// @see https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

/// output the first part of files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub file: Vec<String>,

    /// print the first NUM bytes of each file; with the leading '-', print all but the last NUM bytes of each file
    #[arg(short = 'c')]
    pub bytes: Option<String>,

    /// print the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file
    #[arg(short = 'n', long, default_value = "10")]
    pub lines: String,

    /// never print headers giving file names
    #[arg(short = 'q', long, visible_alias = "silent", default_value_t = false)]
    pub quiet: bool,

    /// always print headers giving file names
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// line delimiter is NUL, not newline
    #[arg(short = 'z', long, default_value_t = false)]
    pub zero_terminated: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
