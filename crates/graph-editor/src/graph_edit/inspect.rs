use crate::{
    graph_edit::{
        GraphEdit,
        edge::{EdgeInspector, GraphEdge},
        node::{GraphNode, NodeInspector},
    },
    visual::{edge::EdgeVisual, node::NodeVisual},
};
use nodus_graph::{
    graph::error::InsertError,
    sets::{edge::EdgeId, vertex::NodeId},
};

impl GraphEdit {
    pub fn inspect_node(&mut self, id: &NodeId) -> Option<NodeInspector<'_>> {
        let graph = self.graph.as_mut()?;

        let visual = self.visual_graph.nodes.get_mut(id)?;

        let data = graph.inspect_node(id)?;

        Some(NodeInspector {
            visual_data: visual,
            underlying_data: data,
        })
    }

    pub fn inspect_edge(&mut self, id: &EdgeId) -> Option<EdgeInspector<'_>> {
        let graph = self.graph.as_mut()?;

        let visual = self.visual_graph.edges.get_mut(id)?;

        let data = graph.inspect_edge(id)?;

        Some(EdgeInspector {
            visual_data: visual,
            underlying_data: data,
        })
    }

    pub fn default_node(&self) -> Option<GraphNode> {
        let graph = self.graph.as_ref()?;

        let visual = NodeVisual::default();

        let data = graph.default_node();

        Some(GraphNode {
            visual_data: visual,
            underlying_data: data,
        })
    }

    pub fn default_edge(&self) -> Option<GraphEdge> {
        let graph = self.graph.as_ref()?;

        let visual = EdgeVisual::default();

        let data = graph.default_edge();

        Some(GraphEdge {
            visual_data: visual,
            underlying_data: data,
        })
    }

    pub fn insert_node(&mut self, value: GraphNode) -> Option<Result<NodeId, InsertError>> {
        let GraphNode {
            visual_data,
            underlying_data,
        } = value;

        let node_id = match self.graph.as_mut()?.insert_node(underlying_data) {
            Ok(id) => id,
            Err(err) => return Some(Err(err)),
        };

        self.visual_graph.nodes.insert(node_id, visual_data);

        Some(Ok(node_id))
    }

    pub fn insert_edge(&mut self, value: GraphEdge) -> Option<Result<EdgeId, InsertError>> {
        let GraphEdge {
            visual_data,
            underlying_data,
        } = value;

        let node_id = match self.graph.as_mut()?.insert_edge(underlying_data) {
            Ok(id) => id,
            Err(err) => return Some(Err(err)),
        };

        self.visual_graph.edges.insert(node_id, visual_data);

        Some(Ok(node_id))
    }

    pub fn validate_node(&self, value: &GraphNode) -> Option<Result<(), InsertError>> {
        Some(
            self.graph
                .as_ref()?
                .validate_node(value.underlying_data.as_reflect()),
        )
    }

    pub fn validate_edge(&self, value: &GraphEdge) -> Option<Result<(), InsertError>> {
        Some(
            self.graph
                .as_ref()?
                .validate_edge(value.underlying_data.as_reflect()),
        )
    }
}
