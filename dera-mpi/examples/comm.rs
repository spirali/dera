
pub fn main() {
    let (server, worker) = dera_mpi::init_mpi_transport().unwrap();

    if let Some(s) = server {
        println!("I am server")
    };

    println!("I am worker {}", worker.worker_id());
}