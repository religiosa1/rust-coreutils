use ctrlc::set_handler;
use notify::event::ModifyKind;
use notify::{EventKind, RecursiveMode, Watcher};
use std::error::Error;
use std::path::Path;
use std::sync::mpsc::channel;

use crate::args::Args;
use crate::tail::tail;

pub fn follow(args: &Args) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    for file in &args.file {
        watcher.watch(Path::new(file), RecursiveMode::NonRecursive)?;
        tail(&args, file)?;
    }

    // Set up a Ctrl-C handler to stop the watcher
    set_handler(move || {
        std::process::exit(0);
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if let EventKind::Modify(ModifyKind::Any) = event.kind {
                    if let Some(pb) = event.paths.first() {
                        if let Some(name) = pb.to_str() {
                            // TODO normalize name to the initial
                            tail(&args, name)?;
                        // rewrite this bullshit
                        } else {
                            eprintln!("Empty file name in event");
                        }
                    } else {
                        eprintln!("Empty file name in event");
                    }
                }
            }
            Ok(Err(e)) => eprintln!("watch error: {:?}", e),
            Err(e) => eprintln!("signal error: {:?}", e),
        };
    }
}
