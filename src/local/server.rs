use std::rc::Rc;
use std::sync::Arc;
use futures::{Future, Stream};
use failure::{Error, format_err};

use crate::{WorkerId, MessageTag, ObjectId, Object};
use crate::ServerTransportEvent;
use crate::ServerTransport;
use bytes::BytesMut;


pub struct LocalServerTransport {

}

impl LocalServerTransport {
    pub fn new() -> Self {
        LocalServerTransport {}
    }
}

impl ServerTransport for LocalServerTransport {
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
        unimplemented!()
    }

    fn start(&mut self) -> Result<Box<Stream<Item=ServerTransportEvent, Error=Error>>, Error> {
        unimplemented!()
    }
}
