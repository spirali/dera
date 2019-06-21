

pub struct MpiServerTransport {
    world: mpi::topology::SystemCommunicator,
}

impl MpiServerTransport {

    pub(crate) fn new(world: mpi::topology::SystemCommunicator) -> Self {
        MpiServerTransport {
            world
        }
    }

}
