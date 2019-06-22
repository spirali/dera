use mpi::environment::Universe;
use mpi::topology::Communicator;
use std::rc::Rc;
use futures::{Future, Stream};
use failure::Error;

use dera::{WorkerId, MessageTag};
use dera::ServerTransportEvent;
use bytes::BytesMut;
use mpi::point_to_point::Destination;


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

    pub fn send_message_to_worker(&self, worker_id: WorkerId, tag: MessageTag, message: BytesMut) {
        let world = self.universe.world();
        let mpi_tag = (tag as mpi::Tag) | crate::common::TAG_MASK_SERVER_TO_WORKER;
        dbg!(mpi_tag);
        world.process_at_rank(worker_id as mpi::topology::Rank).buffered_send_with_tag(&message[..], mpi_tag);
    }
}
