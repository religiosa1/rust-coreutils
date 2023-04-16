mod args;
mod follow;
mod tail;

use smol;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
    if let Some(_) = args.follow {
        smol::block_on(async {
            follow::follow(&args).await.unwrap();
        });
    } else {
        for file in &args.file {
            tail::tail(&args, &file)?;
        }
    }
    Ok(())
}
