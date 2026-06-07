use std::{
    fmt::Display,
    sync::{
        Mutex,
        atomic::{AtomicU16, Ordering},
    },
};

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct NodeId(pub u16);

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <u16 as Display>::fmt(&self.0, f)
    }
}

impl NodeId {
    pub fn next(&self) -> Option<NodeId> {
        Some(NodeId(self.0.checked_add(1)?))
    }
}

#[derive(Default)]
pub struct NodeIdPool {
    next: AtomicU16,
    free: Mutex<Vec<u16>>,
}

impl NodeIdPool {
    pub fn alloc(&self) -> NodeId {
        // Try to reuse an ID
        if let Some(id) = self.free.lock().unwrap().pop() {
            return NodeId(id);
        }

        // Otherwise allocate a new one
        let id = self.next.fetch_add(1, Ordering::Relaxed);
        NodeId(id)
    }

    pub fn free(&self, id: NodeId) {
        self.free.lock().unwrap().push(id.0);
    }
}
