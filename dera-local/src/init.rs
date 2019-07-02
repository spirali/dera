
use crate::{LocalServerTransport, LocalWorkerTransport};

use failure::Error;

pub fn init_local_transport() -> Result<(LocalServerTransport, Vec<LocalWorkerTransport>), Error>
{
    Ok(unimplemented!())
}