use dera::WorkerId;
use mpi::topology::Communicator;
use mpi::environment::Universe;
use std::rc::Rc;


pub struct MpiWorkerTransport {
    universe: Rc<Universe>
}


impl MpiWorkerTransport {

    pub(crate) fn new(universe: Rc<Universe>) -> Self {
        MpiWorkerTransport {
            universe
        }
    }

}

impl MpiWorkerTransport {

    pub fn worker_id(&self) -> WorkerId {
        self.universe.world().rank() as WorkerId
    }

}
