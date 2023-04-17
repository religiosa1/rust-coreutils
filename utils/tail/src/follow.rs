use ctrlc::set_handler;
use futures::{
    channel::mpsc::{channel, Receiver},
    select, SinkExt, StreamExt,
};

use notify::{
    event::ModifyKind, Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::error::Error;
use std::path::Path;

use crate::args::Args;
use crate::tail::tail;

use futures::FutureExt;
use smol::Timer;
use std::time::Duration;

use crate::pid::PidChecker;

pub async fn follow(args: &Args) -> Result<(), Box<dyn Error>> {
    set_handler(move || {
        std::process::exit(0);
    })
    .unwrap();
    for file in &args.file {
        // Imediately output file's tail on first launch
        tail(&args, file)?;
    }
    if let Some(_) = args.pid {
        select! {
            r = poll_pid(&args).fuse() => r?,
            r = listen_to_fs_event(&args).fuse() => r?,
        }
    } else {
        listen_to_fs_event(&args).await?;
    }
    Ok(())
}

async fn poll_pid(args: &Args) -> Result<(), Box<dyn Error>> {
    let period = Duration::from_secs_f32(args.sleep_interval);
    let checker = PidChecker::new(args.pid.unwrap()).unwrap();
    while checker.check_pid() {
        Timer::interval(period).await;
    }
    Ok(())
}

async fn listen_to_fs_event(args: &Args) -> Result<(), Box<dyn Error>> {
    let (mut watcher, mut rx) = async_watcher()?;

    for file in &args.file {
        watcher.watch(Path::new(file), RecursiveMode::NonRecursive)?;
    }
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                if let EventKind::Modify(ModifyKind::Any) = event.kind {
                    let first_path = event.paths.first().and_then(|pb| pb.to_str());
                    if let Some(name) = first_path {
                        tail(&args, name)?;
                    } else {
                        eprintln!("Empty file name in event");
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}
