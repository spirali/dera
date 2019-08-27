
use std::collections::HashSet;

use super::object::ObjectInfoRef;
use crate::common::WrappedRcRefCell;

pub type WorkerId = u32;


pub struct Worker {
    pub worker_id: WorkerId,
    pub fullname: String, /* MPI -> hostname/rank, TCP -> hostname:port */
    pub objects: HashSet<ObjectInfoRef>,
}

pub type WorkerRef = WrappedRcRefCell<Worker>;
