
use crate::{MpiServerTransport, MpiWorkerTransport};

use mpi::topology::Communicator;
use failure::{Error, bail};
use std::rc::Rc;

pub fn init_mpi_transport() -> Result<(Option<MpiServerTransport>, MpiWorkerTransport), Error> {
    let init = mpi::initialize_with_threading(mpi::Threading::Multiple);

    let (universe, threading) = match init {
        None => bail!("MPI alread initialized"),
        Some(pair) => pair
    };

    if threading != mpi::Threading::Multiple {
        bail!("Insufficient level of threading");
    }

    let universe = Rc::new(universe);

    let world = universe.world();
    log::debug!("Initilazing dera-mpi, world rank {}", world.rank());
    let server = if world.rank() == 0 {
        Some(MpiServerTransport::new(universe.clone()))
    } else {
        None
    };

    let worker = MpiWorkerTransport::new(universe);
    Ok((server, worker))
}