
use futures;
use futures::stream::Stream;
use futures::future::Future;

pub fn main() {
    let (server, worker) = dera_mpi::init_mpi_transport().unwrap();

    if let Some(s) = server {
        println!("I am server");
        dbg!("SENDING 1");
        for i in 0..1000 {
            s.send_message_to_worker(0, 312, "Abc".into());
        }
        dbg!("SENDING 2");
        s.send_message_to_worker(1, 123, "Abc".into());
    };

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


    w_future.wait();
    //futures::executor::run(w_future);
}