
use futures;
use futures::stream::Stream;
use futures::future::Future;

pub fn main() {
    let (server, worker) = dera_mpi::init_mpi_transport().unwrap();

    let s = if let Some(mut s) = server {
        println!("I am server");
        dbg!("SENDING 1");
        for _ in 0..2 {
            s.send_message_to_worker(0, 312, "Abc".into());
        }
        dbg!("SENDING 2");
        s.send_message_to_worker(1, 123, "Fff".into());
        Some(s)
    }

    let w_future = worker.start().unwrap().for_each(|event| {
        match event {
            dera::WorkerTransportEvent::ServerMessage(tag, data) => {
                println!("XXX {} {:?}", tag, data);
            }
            _ => { panic!("Unknown event"); }
        }
        Ok(())
    });
    println!("I am worker {}", worker.worker_id());


    w_future.wait().unwrap();
    //futures::executor::run(w_future);
}