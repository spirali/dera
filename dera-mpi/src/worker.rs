use dera::WorkerId;
use mpi::topology::Communicator;
use mpi::environment::Universe;
use mpi::point_to_point::Source;
use std::rc::Rc;
use futures::{Future, Stream};
use failure::{Error, format_err};


use dera::WorkerTransportEvent;
use dera::MessageTag;

use crate::common;
use crate::core::Core;


pub struct MpiWorkerTransport {
    core: Rc<Core>
}


impl MpiWorkerTransport {

    pub(crate) fn new(core: Rc<Core>) -> Self {
        MpiWorkerTransport {
            core
        }
    }

}

impl MpiWorkerTransport {

    pub fn worker_id(&self) -> WorkerId {
        self.core.rank() as WorkerId
    }

    pub fn start(&self) -> Result<Box<Stream<Item=WorkerTransportEvent, Error=Error>>, Error>
    {
        let (sender, receiver) = futures::sync::mpsc::unbounded();
        let world = self.core.world();
        std::thread::spawn(move || {
            loop {
                let (data, status) = world.any_process().receive_vec::<u8>();
                match status.tag() {
                    tag if tag & common::TAG_MASK_SERVER_TO_WORKER != 0 => {
                        let user_tag = (tag & !common::TAG_MASK_SERVER_TO_WORKER) as MessageTag;
                        sender.send(WorkerTransportEvent::ServerMessage(user_tag, data.into())).unwrap();
                    }
                    tag => {
                        panic!("Unexpected tag: {}", tag);
                    }
                };
            }
        });
        Ok(Box::new(receiver.map_err(|()| format_err!("Receiver failed"))))
    }

}
