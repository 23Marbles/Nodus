#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct NodeId(pub u16);

impl NodeId {
    pub fn next(&self) -> Option<NodeId> {
        Some(NodeId(self.0.checked_add(1)?))
    }
}
