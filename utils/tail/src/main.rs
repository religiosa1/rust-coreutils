mod args;
mod follow;
mod tail;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
    if let Some(_) = args.follow {
        follow::follow(&args)?;
    } else {
        for file in &args.file {
            tail::tail(&args, &file)?;
        }
    }
    Ok(())
}
