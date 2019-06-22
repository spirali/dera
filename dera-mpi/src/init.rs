
use crate::{MpiServerTransport, MpiWorkerTransport};

use crate::core::Core;
use mpi::topology::Communicator;
use failure::{Error, bail};
use std::rc::Rc;

pub fn init_mpi_transport() -> Result<(Option<MpiServerTransport>, MpiWorkerTransport), Error> {
    let init = mpi::initialize_with_threading(mpi::Threading::Multiple);

    let (mut universe, threading) = match init {
        None => bail!("MPI alread initialized"),
        Some(pair) => pair
    };

    if threading != mpi::Threading::Multiple {
        bail!("Insufficient level of threading");
    }

    let core = Rc::new(Core::new(universe));

    let rank = core.rank();
    log::debug!("Initilazing dera-mpi, world rank {}", rank);
    let server = if rank == 0 {
        Some(MpiServerTransport::new(core.clone()))
    } else {
        None
    };

    let worker = MpiWorkerTransport::new(core);
    Ok((server, worker))
}