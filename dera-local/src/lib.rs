
mod server;
mod worker;
mod init;

use server::LocalServerTransport;
use worker::LocalWorkerTransport;
use init::init_local_transport;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
