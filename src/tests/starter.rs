
use crate::mpi::init_mpi_transport;
use crate::{ServerTransport, WorkerTransport};
use crate::{ServerManagerRef, WorkerManagerRef};

pub fn starter<F: Fn(Option<ServerManagerRef>, Option<WorkerManagerRef>)>
        (test_fn: F) {
    let (server, worker) = init_mpi_transport().unwrap();
    let worker = Some(worker);

    let s_manager = server.map(|server| ServerManagerRef::new(Box::new(server)));
    let w_manager = worker.map(|worker| WorkerManagerRef::new(Box::new(worker)));

    test_fn(s_manager, w_manager);
}
