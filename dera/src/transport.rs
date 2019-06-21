
use bytes::{BytesMut};
use std::rc::Rc;
use futures::{Future, Stream};
use failure::Error;

use crate::{ObjectId, ObjectInfoRef, WorkerId, Object, WorkerRef};


pub type MessageTag = u16;

pub enum ServerTransportEvent {
    WorkerMessage(WorkerId, MessageTag, BytesMut),
    NewWorker(WorkerRef),
    LostWorker(WorkerRef),
}


pub trait ServerTransport {

    fn fetch_object(&self, worker_id: WorkerId, object_id: ObjectId) -> Box<Future<Item=Object, Error=Error>>;
    fn fetch_object_part(&self, worker_id: WorkerId, object_id: ObjectId, offset: u64, size: u64) -> Box<Future<Item=BytesMut, Error=Error>>;
    fn push_object(worker_id: WorkerId, object: Rc<Object>);

    fn send_message_to_worker(&self, worker_id: WorkerId, tag: MessageTag, message: BytesMut);
    fn start(&self) -> Result<Box<Stream<Item=ServerTransportEvent, Error=Error>>, Error>;
}


pub enum WorkerTransportEvent {
    ServerMessage(MessageTag, BytesMut),
    ObjectRequest(ObjectRequest),
    LostConnection,
}


pub struct ObjectRequest {
    pub object_id: ObjectId,
    pub response: (), /* oneshot? */
}


pub trait WorkerTransport {

    fn fetch_object(&self, worker_id: WorkerId, object_id: ObjectId) -> Box<Future<Item=Object, Error=Error>>;

    fn send_message_to_server(&self, tag: MessageTag, message: BytesMut);
    fn start(&self) -> Result<Box<Stream<Item=(), Error=Error>>, Error>;
}