
use crate::mpi::init_mpi_transport;
use crate::local::LocalTransport;
use crate::{ServerTransport, WorkerTransport};
use crate::{ServerManagerRef, WorkerManagerRef};
use std::env;

pub fn starter<F: Fn(Option<ServerManagerRef>, Option<WorkerManagerRef>)>
        (test_fn: F) {
    let mut args = env::args().skip(1);
    match (args.next().as_ref(), args.next().as_ref()) {
        (Some(s), None) if s == "mpi" => {
            let (server, worker) = init_mpi_transport().unwrap();
            let worker = Some(worker);
            let s_manager = server.map(|server| ServerManagerRef::new(Box::new(server)));
            let w_manager = worker.map(|worker| WorkerManagerRef::new(Box::new(worker)));
            test_fn(s_manager, w_manager);
        },
        (Some(s), Some(t)) if s == "local" => {
            let n_processes = t.parse::<u32>().unwrap();
            /*let (server, workers) = init_local_transport().unwrap();
            let (server, worker) = init_mpi_transport().unwrap();
            let worker = Some(worker);
            let s_manager = server.map(|server| ServerManagerRef::new(Box::new(server)));
            let w_manager = worker.map(|worker| WorkerManagerRef::new(Box::new(worker)));*/
            let core = LocalTransport::init();

            let c = core.clone();
            let server_th = std::thread::spawn(move || {
                let server = c.start_server();
                test_fn(Some(ServerManagerRef::new(Box::new(server))), None);
            });



        },
        (Some(_), _) => {
            println!("Invalid mode");
            std::process::exit(1)
        },
        (None, _) => {
            println!("No mode provided");
            std::process::exit(1)
        }
    };
}
