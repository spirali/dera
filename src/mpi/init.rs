
use super::{MpiServerTransport, MpiWorkerTransport};

use crate::{ServerTransportEvent, WorkerId};
use super::core::Core;
use mpi::topology::Communicator;
use failure::{Error, bail};
use std::rc::Rc;
use std::sync::Arc;


pub fn init_mpi_transport() -> Result<(Option<MpiServerTransport>, MpiWorkerTransport), Error> {
    let init = mpi::initialize_with_threading(mpi::Threading::Multiple);

    let (mut universe, threading) = match init {
        None => bail!("MPI alread initialized"),
        Some(pair) => pair
    };

    if threading != mpi::Threading::Multiple {
        bail!("Insufficient level of threading");
    }

    let core = Arc::new(Core::new(universe));
    let rank = core.rank();
    log::debug!("Initilazing dera-mpi, world rank {}", rank);
    let (server, server_sender) = if rank == 0 {
        let (server_sender, sender_receiver) = futures::sync::mpsc::unbounded();
        let transport = MpiServerTransport::new(core.clone(), sender_receiver);
        for r in 0..core.world().size() {
            let fullname = format!("rank{}/TODO", r);
            server_sender.unbounded_send(ServerTransportEvent::NewWorker(r as WorkerId, fullname));
        }
        (Some(transport), Some(server_sender))
    } else {
        (None, None)
    };

    let worker = MpiWorkerTransport::new(core, server_sender);
    Ok((server, worker))
}