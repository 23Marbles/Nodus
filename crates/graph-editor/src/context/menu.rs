use nodus_graph::sets::{edge::EdgeId, vertex::NodeId};

#[derive(Debug)]
pub enum ContextMenuKind {
    Node { id: NodeId },
    Edge { id: EdgeId },
    Graph,
}
