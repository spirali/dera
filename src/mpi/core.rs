use mpi::environment::Universe;
use mpi::topology::Communicator;
use mpi::topology::{Rank, SystemCommunicator};

pub(crate) struct Core {
    universe: Universe
}

impl Core {
    pub fn new(universe: Universe) -> Self {
        Core { universe }
    }

    pub fn world(&self) -> SystemCommunicator {
        self.universe.world()
    }

    pub fn rank(&self) -> Rank {
        self.universe.world().rank()
    }
}