use mpi::environment::Universe;
use mpi::topology::{Rank, Communicator};
use std::rc::Rc;
use futures::{Future, Stream};
use failure::Error;

use crate::{WorkerId, MessageTag};
use crate::ServerTransportEvent;
use bytes::BytesMut;
use mpi::point_to_point::Destination;

use super::core::Core;
use super::rqm::RequestManager;

pub struct MpiServerTransport {
    core: Rc<Core>,
    request_manager: RequestManager,
}


impl MpiServerTransport {

    pub(crate) fn new(core: Rc<Core>) -> Self {
        assert!(core.rank() == 0);
        MpiServerTransport {
            core,
            request_manager: RequestManager::new(),
        }
    }

    pub fn send_message_to_worker(&mut self, worker_id: WorkerId, tag: MessageTag, message: Vec<u8>) {
        let mpi_tag = (tag as mpi::Tag) | crate::mpi::common::TAG_MASK_SERVER_TO_WORKER;
        self.request_manager.send(worker_id as Rank, mpi_tag, message);
        //self.request_manager.check();
        //self.core.send_message(worker_id as Rank, mpi_tag, message);
        //world.process_at_rank().buffered_send_with_tag(&message[..], mpi_tag);
    }


}
