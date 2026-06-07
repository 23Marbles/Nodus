use std::fmt::Display;

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
