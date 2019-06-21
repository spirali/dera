
use crate::{MpiServerTransport, MpiWorkerTransport};

use mpi::topology::Communicator;
use failure::{Error, bail};

pub fn init_mpi_transport() -> Result<(Option<MpiServerTransport>, MpiWorkerTransport), Error> {
    let init = mpi::initialize_with_threading(mpi::Threading::Multiple);

    let (universe, threading) = match init {
        None => bail!("MPI alread initialized"),
        Some(pair) => pair
    };

    if threading != mpi::Threading::Multiple {
        bail!("Insufficient level of threading");
    }

    let world = universe.world();

    log::debug!("Initilazing dera-mpi, world rank {}", world.rank());

    let server = if world.rank() == 0 {
        let world = universe.world();
        Some(MpiServerTransport::new(world))
    } else {
        None
    };

    let worker = MpiWorkerTransport::new(world);
    Ok((server, worker))
}