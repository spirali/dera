
use mpi::{Tag};
use mpi::topology::{Rank, Communicator};
use bytes::BytesMut;
use std::ffi::c_void;
use std::cmp::Reverse;


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
        dbg!(&data);
        self.data.push(data);
        for x in &self.data {
            dbg!(x);
        }
    }

    pub fn wait(&mut self) {
        unsafe {
            mpi_sys::MPI_Waitall(self.requests.len() as i32, self.requests.as_mut_ptr(), mpi_sys::RSMPI_STATUSES_IGNORE);
        }
        self.requests.clear();
        self.data.clear();
    }


    pub fn check(&mut self) {
        if self.requests.is_empty() {
            return;
        }
        let count = self.requests.len();
        let mut out_count = 0;

        // TODO: For small number of requests, does not allocate
        let mut out : Vec<i32> = Vec::new();
        out.resize(count, 0);

        unsafe {
            mpi_sys::MPI_Testsome(count as i32, self.requests.as_mut_ptr(), &mut out_count, out.as_mut_ptr(), mpi_sys::RSMPI_STATUSES_IGNORE);
        }

        let out_count = out_count as usize;
        assert!(out_count <= out.len());

        if out_count == 0 {
            return;
        }

        out.resize(out_count, 0);
        out.sort_by_key(|&num| Reverse(num));

        for i in out {
            let i = i as usize;
            if i - 1 == self.requests.len() {
                self.requests.pop();
                self.data.pop();
            } else {
                self.requests[i] = self.requests.pop().unwrap();
                self.data[i] = self.data.pop().unwrap();
            }
        }
    }
}


impl Drop for RequestManager {
    fn drop(&mut self) {
        self.wait();
    }
}