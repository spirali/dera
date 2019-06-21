mod server;
mod worker;
mod init;

pub use init::init_mpi_transport;
pub use server::MpiServerTransport;
pub use worker::MpiWorkerTransport;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let (x, t) = mpi::initialize_with_threading(mpi::Threading::Multiple).unwrap();
        println!("OUTPUT: {:?}", t);
    }
}
