
use bytes::{BytesMut};
use std::rc::Rc;
use futures::{Future, Stream};
use futures::future;
use failure::Error;

use super::transport::{WorkerTransport, MessageTag};
use crate::common::WrappedRcRefCell;
use crate::{WorkerId, ObjectId, Object, WorkerRef};

const TAG_CUSTOM_MESSAGE : MessageTag = 1;

#[derive(Debug)]
pub enum WorkerEvent {
    OnMessage(WorkerId, BytesMut)
}

pub struct WorkerManager {
    transport: Box<dyn WorkerTransport>,
}

pub type WorkerManagerRef = WrappedRcRefCell<WorkerManager>;

impl WorkerManager {

}

impl WorkerManagerRef {

    pub fn new(transport: Box<dyn WorkerTransport>) -> Self {
        WrappedRcRefCell::wrap(WorkerManager {
            transport,
        })
    }

    /// Sends custem message to worker
    pub fn send_message_to_server(&self, message: BytesMut) {
        let manager = self.get();
        manager.transport.send_message_to_server(TAG_CUSTOM_MESSAGE, message);
    }

    /// Fetch object, contact server for address
    pub fn fetch_object(&self, object_id: ObjectId) -> impl Future<Item=Rc<Object>, Error=Error>
    {
        futures::future::ok(unimplemented!())
    }

    /// Push object into system, by default into local storage
    /// In case of memory limitation, then server is contacted where to push object
    pub fn push_object(&self, object: Rc<Object>) {
        unimplemented!();
    }

    /// Start manager
    /// Argument is function that is called for every event
    /// Returns a future that represent running manager. Manager can be stopped by dropping this future.
    pub fn start(&self, on_event: impl Fn(WorkerEvent)) -> Result<impl Future<Item=(), Error=Error>, Error> {
        let manager_ref = self.clone();
        let mut manager = self.get_mut();
        let message_stream = manager.transport.start().unwrap();
        let msg_process = message_stream.for_each(move |event| {
            futures::future::ok(unimplemented!())
        });
        Ok(msg_process)
    }

    /*fn _process_message(&self, worker_id: WorkerId, tag: MessageTag, message: BytesMut) -> impl Future<Item=(), Error=Error> {
        /*match tag {
            TAG_CUSTOM_MESSAGE => {},
            _ => {
                // TODO ERROR
                panic!("Invalid message tag");
            }
        };*/
        futures::future::ok(())
    }*/
}