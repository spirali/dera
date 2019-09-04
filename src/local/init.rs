
use super::{LocalServerTransport, LocalWorkerTransport};
use crate::{ServerTransportEvent, WorkerTransportEvent};

use failure::Error;
use std::sync::Arc;
use std::sync::Mutex;

use futures::sync::mpsc::Sender;

struct LocalTransportInner {
    server_sender: Option<Sender<ServerTransportEvent>>,
    worker_senders: Vec<Sender<WorkerTransportEvent>>,
}

#[derive(Clone)]
pub struct LocalTransport {
    inner: Arc<Mutex<LocalTransportInner>>
}

impl LocalTransport {
    pub fn init() -> Self {
        LocalTransport {
            inner: Arc::new(Mutex::new(LocalTransportInner {
                server_sender: None,
                worker_senders: Vec::new(),
            }))
        }
    }

    pub fn start_server(&self) -> LocalServerTransport {
        LocalServerTransport::new()
    }
}

/*pub fn init_local_server_transport() -> Result<(LocalServerTransport, Vec<LocalWorkerTransport>), Error>
{
    Ok(unimplemented!())
}*/