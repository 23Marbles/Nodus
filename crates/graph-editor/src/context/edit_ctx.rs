use crate::{context::menu::ContextMenuKind, graph_edit::GraphEdit};

pub struct GraphContext<'a> {
    pub graph_edit: &'a mut GraphEdit,
    pub kind: ContextMenuKind,
}

impl<'a> GraphContext<'a> {
    pub fn from_graph(graph: &'a mut GraphEdit) -> GraphContext<'a> {
        GraphContext {
            graph_edit: graph,
            kind: ContextMenuKind::Graph,
        }
    }
}
