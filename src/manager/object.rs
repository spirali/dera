
use crate::WorkerId;
use crate::common::WrappedRcRefCell;

pub type ObjectId = u64;

pub struct ObjectInfo {
    object_id: ObjectId,
    pinned: Vec<WorkerId>
}

pub type ObjectInfoRef = WrappedRcRefCell<ObjectInfo>;

impl ObjectInfoRef {
    pub fn new(object_id: ObjectId) -> Self {
        ObjectInfoRef::wrap(ObjectInfo {
            object_id,
            pinned: Default::default(),
        })
    }
}


pub struct Object {

}