

//use dera::tests::starter;
use dera::{ServerTransport, WorkerTransport, ServerEvent, WorkerEvent};
use dera::{ServerManagerRef, WorkerManagerRef};
use futures::future::Future;
use futures::unsync::oneshot;
use tokio;
use tokio::executor::current_thread;
use dera::tests::starter::starter;
use bytes::BufMut;



fn start_ping(server_mng: Option<ServerManagerRef>, worker_mng: Option<WorkerManagerRef>) {
    let (stop_sender, stop_receiver) = oneshot::channel::<()>();
    let mut stop_sender = Some(stop_sender);
    let sfuture : Option<_> = server_mng.map(|sm| {
        let s = sm.clone();
        let mut counter = 0;
        let mut workers = 0;
        sm.start(move |msg| {
            match msg {
                ServerEvent::NewWorker(worker) => {
                    println!("New worker {}/{}", worker.worker_id(), worker.fullname());
                    let m = format!("Hello");
                    workers += 1;
                    if (worker.worker_id() == 0) {
                        s.send_message_to_worker(worker.worker_id(), m.into_bytes());
                    }
                },
                ServerEvent::OnMessage(worker_id, message) => {

                        dbg!(&message, worker_id);
                        counter += 1;
                        if counter < 10 {
                            let w_id = (worker_id + 1) % workers;
                            s.send_message_to_worker(w_id, message.to_vec());
                        } else {
                            stop_sender.take().unwrap().send(()).unwrap();
                        }

                },
            }
            let x = &s;
        }).unwrap().map_err(|e| { panic!("{}", e) })
    });

    let wfuture : Option<_> = worker_mng.map(|wm| {
        let mut w = wm.clone();
        wm.start(move |msg| {
            println!("EVENT on worker: {}", w.worker_id());
            match msg {
                WorkerEvent::OnMessage(msg) => {
                    println!("new message from server: {:?}", msg);
                    let mut v: Vec<u8> = msg.to_vec();
                    let back_msg = format!("{}", w.worker_id());
                    v.append(&mut back_msg.into_bytes());
                    //msg.put(b'!');
                    /*println!("New worker {}/{}", worker.worker_id(), worker.fullname());
                    s.send_message_to_worker(worker.worker_id(), b"Hello".to_vec());*/
                    w.send_message_to_server(v);
                },
                x => {
                    dbg!(x);
                }
            }
        }).unwrap().map_err(|e| { panic!("{}", e) })
    });

    println!("Init finished ...");
    match (sfuture, wfuture) {
        (None, Some(f)) => { current_thread::block_on_all(f).unwrap(); },
        (Some(f), None) => { current_thread::block_on_all(f).unwrap(); },
        (Some(f1), Some(f2)) => { current_thread::block_on_all(f1.select(f2).select2(stop_receiver).map(|_| ()).map_err(|_| ())).unwrap(); },
        (None, None) => unreachable!()
    };
    println!("Terminating ...");
}


fn main() {
    starter(start_ping);
}