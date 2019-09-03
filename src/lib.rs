//pub mod server;
//pub mod transport;

pub mod common;
pub mod manager;
pub mod tests;

#[cfg(feature = "local-transport")]
mod local;

#[cfg(feature = "mpi-transport")]
mod mpi;



pub use self::manager::worker::{WorkerId, WorkerRef};
pub use self::manager::object::{ObjectId, ObjectInfo, ObjectInfoRef, Object};
pub use self::manager::transport::{ServerTransport, ServerTransportEvent, WorkerTransport, WorkerTransportEvent, MessageTag};
pub use self::manager::s_manager::{ServerManagerRef, ServerEvent};
pub use self::manager::w_manager::{WorkerManagerRef, WorkerEvent};