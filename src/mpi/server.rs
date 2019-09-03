use mpi::environment::Universe;
use mpi::topology::{Rank, Communicator};
use std::rc::Rc;
use std::sync::Arc;
use futures::{Future, Stream};
use failure::{Error, format_err};

use crate::{WorkerId, MessageTag, ObjectId, Object};
use crate::ServerTransportEvent;
use crate::ServerTransport;
use bytes::BytesMut;
use mpi::point_to_point::Destination;
use mpi::point_to_point::Source;

use super::core::Core;
use super::rqm::RequestManager;

pub struct MpiServerTransport {
    core: Arc<Core>,
    request_manager: RequestManager,
    receiver: Option<futures::sync::mpsc::UnboundedReceiver<ServerTransportEvent>>
}


impl MpiServerTransport {

    pub(crate) fn new(core: Arc<Core>, receiver: futures::sync::mpsc::UnboundedReceiver<ServerTransportEvent>) -> Self {
        assert!(core.rank() == 0);
        MpiServerTransport {
            core,
            request_manager: RequestManager::new(),
            receiver: Some(receiver),
        }
    }
}


impl ServerTransport for MpiServerTransport {
    fn fetch_object(&self, worker_id: WorkerId, object_id: ObjectId) -> Box<Future<Item=Object, Error=Error>> {
        unimplemented!()
    }

    fn fetch_object_part(&self, worker_id: WorkerId, object_id: ObjectId, offset: u64, size: u64) -> Box<Future<Item=BytesMut, Error=Error>> {
        unimplemented!()
    }

    fn push_object(&self, worker_id: WorkerId, object: Rc<Object>) {
        unimplemented!()
    }

    fn send_message_to_worker(&mut self, worker_id: WorkerId, tag: MessageTag, message: Vec<u8>) {
        let mpi_tag = (tag as mpi::Tag) | crate::mpi::common::TAG_MASK_SERVER_TO_WORKER;
        self.request_manager.send(worker_id as Rank, mpi_tag, message);
    }

    fn start(&mut self) -> Result<Box<Stream<Item=ServerTransportEvent, Error=Error>>, Error> {
        Ok(Box::new(self.receiver.take().unwrap().map_err(|()| unreachable!())))
    }
}
