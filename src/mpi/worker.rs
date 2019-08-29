use crate::WorkerId;
use mpi::topology::Communicator;
use mpi::environment::Universe;
use mpi::point_to_point::Source;
use std::rc::Rc;
use futures::{Future, Stream};
use failure::{Error, format_err};


use crate::{WorkerTransportEvent, WorkerTransport, ObjectId, Object};
use crate::{MessageTag, ServerTransportEvent};

use crate::common;
use super::core::Core;
use bytes::BytesMut;


pub struct MpiWorkerTransport {
    core: Rc<Core>,
    server_sender: Option<futures::sync::mpsc::UnboundedSender<ServerTransportEvent>>,
}


impl MpiWorkerTransport {

    pub(crate) fn new(core: Rc<Core>, server_sender: Option<futures::sync::mpsc::UnboundedSender<ServerTransportEvent>>) -> Self {
        MpiWorkerTransport {
            core, server_sender
        }
    }

}

impl MpiWorkerTransport {

    pub fn worker_id(&self) -> WorkerId {
        self.core.rank() as WorkerId
    }

}


impl WorkerTransport for MpiWorkerTransport {

    fn fetch_object(&self, worker_id: WorkerId, object_id: ObjectId) -> Box<Future<Item=Object, Error=Error>> {
        unimplemented!()
    }

    fn send_message_to_server(&self, tag: MessageTag, message: BytesMut) {
        unimplemented!();
    }

    fn start(&mut self) -> Result<Box<Stream<Item=WorkerTransportEvent, Error=Error>>, Error>
    {
        let (sender, receiver) = futures::sync::mpsc::unbounded();
        let world = self.core.world();
        std::thread::spawn(move || {
            loop {
                let (data, status) = world.any_process().receive_vec::<u8>();
                match status.tag() {
                    tag if tag & super::common::TAG_MASK_SERVER_TO_WORKER != 0 => {
                        let user_tag = (tag & !super::common::TAG_MASK_SERVER_TO_WORKER) as MessageTag;
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

    fn worker_id(&self) -> WorkerId {
        self.core.rank() as WorkerId
    }
}