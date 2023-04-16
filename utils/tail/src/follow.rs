use ctrlc::set_handler;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};

use notify::{
    event::ModifyKind, Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::error::Error;
use std::path::Path;

use crate::args::Args;
use crate::tail::tail;

use futures::future;
use smol::Timer;
use std::time::Duration;

pub async fn follow(args: &Args) -> Result<(), Box<dyn Error>> {
    set_handler(move || {
        std::process::exit(0);
    })
    .unwrap();
    if let Some(_) = args.pid {
        let pid_poller = poll_pid(&args);
        let fs_events = listen_to_fs_event(&args);

        let (_, result_fs) = future::join(pid_poller, fs_events).await;
        result_fs?;
    } else {
        listen_to_fs_event(&args).await?;
    }
    Ok(())
}

async fn poll_pid(args: &Args) -> Result<(), Box<dyn Error>> {
    loop {
        Timer::interval(Duration::from_secs_f32(args.sleep_interval)).await;
        check_pid().await;
    }
}

async fn listen_to_fs_event(args: &Args) -> Result<(), Box<dyn Error>> {
    let (mut watcher, mut rx) = async_watcher()?;

    for file in &args.file {
        watcher.watch(Path::new(file), RecursiveMode::NonRecursive)?;
        // Imediately output file's tail on first launch
        tail(&args, file)?;
    }
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
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

async fn check_pid() {
    // Do something here...
    println!("Hello from pid action!");
}
