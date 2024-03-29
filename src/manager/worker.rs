
use std::collections::HashSet;

use super::object::ObjectInfoRef;
use crate::common::WrappedRcRefCell;
use std::fmt;

pub type WorkerId = u32;

pub struct Worker {
    worker_id: WorkerId,
    fullname: String, /* MPI -> hostname/rank, TCP -> hostname:port */
    objects: HashSet<ObjectInfoRef>,
}

pub type WorkerRef = WrappedRcRefCell<Worker>;

impl WorkerRef {
    pub fn new(worker_id: WorkerId, fullname: &str) -> Self {
        WorkerRef::wrap(Worker {
            worker_id, fullname: fullname.to_string(), objects: Default::default(),
        })
    }

    pub fn worker_id(&self) -> WorkerId {
       let worker = self.get();
       worker.worker_id
    }

    pub fn fullname(&self) -> String {
        let worker = self.get();
        worker.fullname.clone()
    }
}


impl fmt::Debug for WorkerRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WorkerRef")
    }
}