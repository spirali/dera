

//use dera::tests::starter;
use dera::{ServerTransport, WorkerTransport};
use dera::{ServerManagerRef, WorkerManagerRef};
use futures::future::Future;
use futures::unsync::oneshot;
use tokio;
use tokio::executor::current_thread;
use dera::tests::starter::starter;


fn start_ping(server_mng: Option<ServerManagerRef>, worker_mng: Option<WorkerManagerRef>) {
    let (stop_sender, stop_receiver) = oneshot::channel::<()>();

    let sfuture : Option<_> = server_mng.map(|sm| {
        let s = sm.clone();
        sm.start(move |msg| {
            dbg!(msg);
            let x = &s;
        }).unwrap().map_err(|e| { panic!("{}", e) })
    });

    let wfuture : Option<_> = worker_mng.map(|wm| {
        let w = wm.clone();
        wm.start(move |msg| {
            dbg!();
            let x = &w;
        }).unwrap().map_err(|e| { panic!("{}", e) })
    });

    println!("Init finished ...");
    match (sfuture, wfuture) {
        (None, Some(f)) => { current_thread::block_on_all(f).unwrap(); },
        (Some(f), None) => { current_thread::block_on_all(f).unwrap(); },
        (Some(f1), Some(f2)) => { current_thread::block_on_all(f1.select(f2).map(|_| ()).map_err(|_| ())).unwrap(); },
        (None, None) => unreachable!()
    };
    println!("Terminating ...");
}


fn main() {
    starter(start_ping);
}