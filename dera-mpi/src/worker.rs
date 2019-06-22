use dera::WorkerId;
use mpi::topology::Communicator;
use mpi::environment::Universe;
use mpi::point_to_point::Source;
use std::rc::Rc;
use futures::{Future, Stream};
use failure::Error;


use dera::WorkerTransportEvent;


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


    fn start(&self) -> Result<Box<Stream<Item=WorkerTransportEvent, Error=Error>>, Error>
    {
        let (sender, receiver) = crossbeam::crossbeam_channel::unbounded();

        let world = self.universe.world();
        std::thread::spawn(move || {
            let (data, status) = world.any_process().receive_vec::<u8>();
            sender.send(data).unwrap();
        });
        Ok(unimplemented!())
    }

}
