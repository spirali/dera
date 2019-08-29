
use bytes::{BytesMut};
use std::rc::Rc;
use futures::{Future, Stream};
use futures::future;
use failure::Error;

use super::transport::{ServerTransport, MessageTag, ServerTransportEvent};
use crate::common::WrappedRcRefCell;
use crate::{WorkerId, ObjectId, Object, WorkerRef, ObjectInfoRef};

const TAG_CUSTOM_MESSAGE : MessageTag = 1;

#[derive(Debug)]
pub enum ServerEvent {
    OnMessage(WorkerId, BytesMut),
    NewWorker(WorkerRef),
}

pub struct ServerManager {
    transport: Box<dyn ServerTransport>,
    /// Local worker that lives in the same process as server or None
    local_worker: Option<WorkerRef>,
}

pub type ServerManagerRef = WrappedRcRefCell<ServerManager>;

impl ServerManager {

    fn pick_worker_for_fetch(&self, object_id: ObjectId) -> WorkerId {
        unimplemented!();
    }

}

impl ServerManagerRef {

    pub fn new(transport: Box<dyn ServerTransport>) -> Self {
        WrappedRcRefCell::wrap(ServerManager {
            transport,
            local_worker: None,
        })
    }

    /// Sends custem message to worker
    pub fn send_message_to_worker(&self, worker_id: WorkerId, message: BytesMut) {
        let manager = self.get();
        manager.transport.send_message_to_worker(worker_id, TAG_CUSTOM_MESSAGE, message);
    }

    /// Fetch object, automatically choose worker for download
    pub fn fetch_object(&self, object_id: ObjectId) -> impl Future<Item=Object, Error=Error>
    {
        let manager = self.get();
        let worker_id = manager.pick_worker_for_fetch(object_id);
        manager.transport.fetch_object(worker_id, object_id)
    }

    /// Fetch object, automatically choose worker for download
    pub fn fetch_object_part(&self, object_id: ObjectId, offset: u64, size: u64) -> impl Future<Item=BytesMut, Error=Error>
    {
        let manager = self.get();
        let worker_id = manager.pick_worker_for_fetch(object_id);
        manager.transport.fetch_object_part(worker_id, object_id, offset, size)
    }

    /// Push object into system
    /// May fail if there is are no workers
    pub fn push_object(&self, object: Rc<Object>, placement_hint: Option<WorkerRef>) -> Result<(), Error> {
        // TODO: consider memory contention
        /*let manager = self.get();
        placement_hint.unwrap_or()*/
        unimplemented!();
    }

    pub fn remove_object(&self, object_ref: ObjectInfoRef) {
        unimplemented!();
    }


    //fn push_object(object: Rc<Object>);


    /// Start manager
    /// Argument is function that is called for every message
    /// Returns a future that represent running manager. Manager can be stopped by dropping this future.
    pub fn start(&self, on_event: impl Fn(ServerEvent)) -> Result<impl Future<Item=(), Error=Error>, Error> {
        let manager_ref = self.clone();
        let mut manager = self.get_mut();
        let message_stream = manager.transport.start().unwrap();
        let msg_process = message_stream.for_each(move |event| {
            match event {
                ServerTransportEvent::WorkerMessage(worker_id, TAG_CUSTOM_MESSAGE, msg) => {
                    on_event(ServerEvent::OnMessage(worker_id, msg));
                    future::Either::A(futures::future::ok(()))
                },
                ServerTransportEvent::WorkerMessage(worker_id, tag, msg) => {
                    future::Either::B(manager_ref._process_message(worker_id, tag, msg))
                },
                ServerTransportEvent::NewWorker(worker_id, fullname) => {
                    let worker = WorkerRef::new(worker_id, &fullname);
                    on_event(ServerEvent::NewWorker(worker));
                    future::Either::A(futures::future::ok(()))
                },
                ServerTransportEvent::LostWorker(_) => {
                    unimplemented!()
                }
            }
        });
        Ok(msg_process)
    }

    fn _process_message(&self, worker_id: WorkerId, tag: MessageTag, message: BytesMut) -> impl Future<Item=(), Error=Error> {
        /*match tag {
            TAG_CUSTOM_MESSAGE => {},
            _ => {
                // TODO ERROR
                panic!("Invalid message tag");
            }
        };*/
        futures::future::ok(())
    }

}