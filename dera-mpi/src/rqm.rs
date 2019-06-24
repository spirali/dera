
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
        // TODO: For small allocation, do not allocate

        if self.requests.is_empty() {
            return;
        }
        let count = self.requests.len();
        let mut out_count = 0;
        let mut out : Vec<i32> = Vec::with_capacity(count);
        out.resize(count, 0);
        mpi_sys::MPI_Testsome(count as i32, self.requests.get_ptr(), &out_count, out.get_mut_ptr(), mpi_sys::MPI_STATUSES_IGNORE);
    }
}