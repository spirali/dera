use mpi::environment::Universe;
use mpi::topology::Communicator;
use std::rc::Rc;
use futures::{Future, Stream};
use failure::Error;

use dera::ServerTransportEvent;

pub struct MpiServerTransport {
    universe: Rc<Universe>
}


impl MpiServerTransport {

    pub(crate) fn new(universe: Rc<Universe>) -> Self {
        assert!(universe.world().rank() == 0);
        MpiServerTransport {
            universe
        }
    }

    fn start(&self) -> Result<Box<Stream<Item=ServerTransportEvent, Error=Error>>, Error>
    {
        Ok(unimplemented!())
    }

}
