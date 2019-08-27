

//use dera::tests::starter;
use dera::{ServerTransport, WorkerTransport};
use dera::{ServerManagerRef, WorkerManagerRef};
use futures::unsync::oneshot;

fn start_ping<ST: ServerTransport, WT: WorkerTransport>(server_mng: Option<ServerManagerRef<ST>>, worker_mng: WorkerManagerRef<WT>) {
    let (stop_sender, stop_receiver) = oneshot::channel::<()>();
    if let Some(sm) = server_mng {
        let s = sm.clone();
        let future = sm.start(|msg| {});
    }
}


fn main() {

}