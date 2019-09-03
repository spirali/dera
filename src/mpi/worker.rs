use crate::WorkerId;
use mpi::topology::Communicator;
use mpi::environment::Universe;
use mpi::point_to_point::Source;
use std::rc::Rc;
use std::sync::Arc;
use futures::{Future, Stream};
use failure::{Error, format_err};
use super::rqm::RequestManager;


use crate::{WorkerTransportEvent, WorkerTransport, ObjectId, Object};
use crate::{MessageTag, ServerTransportEvent};

use crate::common;
use super::core::Core;
use bytes::BytesMut;


pub struct MpiWorkerTransport {
    core: Arc<Core>,
    request_manager: RequestManager,
    server_sender: Option<futures::sync::mpsc::UnboundedSender<ServerTransportEvent>>,
}


impl MpiWorkerTransport {

    pub(crate) fn new(core: Arc<Core>, server_sender: Option<futures::sync::mpsc::UnboundedSender<ServerTransportEvent>>) -> Self {
        MpiWorkerTransport {
            core,
            request_manager: RequestManager::new(),
            server_sender
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

    fn send_message_to_server(&mut self, tag: MessageTag, message: Vec<u8>) {
        let mpi_tag = (tag as mpi::Tag) | crate::mpi::common::TAG_MASK_WORKER_TO_SERVER;
        self.request_manager.send(0, mpi_tag, message);
    }

    fn start(&mut self) -> Result<Box<Stream<Item=WorkerTransportEvent, Error=Error>>, Error>
    {
        let (sender, receiver) = futures::sync::mpsc::unbounded();
        let server_sender = self.server_sender.take();
        let core = self.core.clone();
        std::thread::spawn(move || {
            let world = core.world();
            loop {
                let (data, status) = world.any_process().receive_vec::<u8>();
                match status.tag() {
                    tag if tag & super::common::TAG_MASK_SERVER_TO_WORKER != 0 => {
                        let user_tag = (tag & !super::common::TAG_MASK_SERVER_TO_WORKER) as MessageTag;
                        sender.send(WorkerTransportEvent::ServerMessage(user_tag, data.into())).unwrap();
                    },
                    tag if tag & super::common::TAG_MASK_WORKER_TO_SERVER != 0 => {
                        if let Some(s) = &server_sender {
                            let user_tag = (tag & !super::common::TAG_MASK_WORKER_TO_SERVER) as MessageTag;
                            let worker_id = status.source_rank() as WorkerId;
                            s.send(ServerTransportEvent::WorkerMessage(worker_id, user_tag, data.into())).unwrap();
                        } else {
                            panic!("Message for server received on non-server rank");
                        }
                    },
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