


pub struct MpiWorkerTransport {
    world: mpi::topology::SystemCommunicator,
}

impl MpiWorkerTransport {

    pub(crate) fn new(world: mpi::topology::SystemCommunicator) -> Self {
        MpiWorkerTransport {
            world
        }
    }

}
