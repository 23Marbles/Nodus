use std::{
    fmt::Display,
    sync::{
        Mutex,
        atomic::{AtomicU16, Ordering},
    },
};

use crate::sets::vertex::NodeId;

pub struct EdgeData<I: Iterator<Item = NodeId>, M> {
    pub coinident_nodes: I,
    pub meta: M,
}

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EdgeId(pub u16);

impl Display for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <u16 as Display>::fmt(&self.0, f)
    }
}

#[derive(Default)]
pub struct EdgeIdPool {
    next: AtomicU16,
    free: Mutex<Vec<u16>>,
}

impl EdgeIdPool {
    pub fn alloc(&self) -> EdgeId {
        // Try to reuse an ID
        if let Some(id) = self.free.lock().unwrap().pop() {
            return EdgeId(id);
        }

        // Otherwise allocate a new one
        let id = self.next.fetch_add(1, Ordering::Relaxed);
        EdgeId(id)
    }

    pub fn free(&self, id: EdgeId) {
        self.free.lock().unwrap().push(id.0);
    }
}
