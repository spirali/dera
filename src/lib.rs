//pub mod server;
//pub mod transport;

mod common;
mod manager;

#[cfg(feature = "local-transport")]
mod local;

#[cfg(feature = "mpi-transport")]
mod mpi;

#[cfg(test)]
mod tests;

pub use self::manager::worker::{WorkerId, WorkerRef};
pub use self::manager::object::{ObjectId, ObjectInfo, ObjectInfoRef, Object};
pub use self::manager::transport::{ServerTransport, ServerTransportEvent, WorkerTransport, WorkerTransportEvent, MessageTag};
pub use self::manager::s_manager::{ServerManagerRef};
pub use self::manager::w_manager::{WorkerManagerRef};