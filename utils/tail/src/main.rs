use tail::tail_error::TailError;

mod args;
mod tail;

fn main() -> Result<(), TailError> {
    let args = args::Args::parse();
    if let Some(_) = args.follow {
        todo!("notify and watch");
    } else {
        for file in &args.file {
            if args.print_headers {
                print_header(&file);
            }
            tail::tail(&args, &file)?;
        }
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
