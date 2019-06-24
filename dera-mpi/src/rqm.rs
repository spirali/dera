
use mpi::{Tag};
use mpi::topology::{Rank, Communicator};
use bytes::BytesMut;
use std::ffi::c_void;

pub(crate) struct RequestManager {
    requests: Vec<mpi_sys::MPI_Request>,
    data: Vec<Vec<u8>>,
}

impl RequestManager {

    pub fn new() -> Self {
        RequestManager {
            requests: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn send(&mut self, rank: Rank, tag:Tag, data: Vec<u8>) {
        let mut request : mpi_sys::MPI_Request = unsafe { std::mem::uninitialized() };
        unsafe {
            mpi_sys::MPI_Isend(data.as_ptr() as *const c_void, data.len() as i32, mpi_sys::RSMPI_UINT8_T, rank, tag, mpi_sys::RSMPI_COMM_WORLD, &mut request);
        }
        self.requests.push(request);
        self.data.push(data);
    }

    pub fn check(&self) {
        if self.requests.is_empty() {
            return;
        }
    }
}