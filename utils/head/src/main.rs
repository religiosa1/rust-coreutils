mod args;
mod head;
use args::Args;
use head::head;
use head::head_error::HeadError;

fn main() -> Result<(), HeadError> {
    let args = Args::parse();
    println!("{:?}", args);
    let print_headers = (args.file.len() > 1 || args.verbose) && !args.quiet;
    for file in &args.file {
        if print_headers {
            print_header(&file);
        }
        head(&args, &file)?;
    }
    Ok(())
}

fn print_header(name: &str) {
    let name = match name {
        "-" => "standard input",
        _ => name,
    };
    println!("==> {} <==", name);
}
