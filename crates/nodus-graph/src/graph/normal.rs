use std::collections::HashMap;

#[cfg(feature = "reflect")]
use crate::graph::{ReflectEdges, ReflectNodes};
use crate::{
    graph::Graph,
    sets::{edge::EdgeId, vertex::NodeId},
};

#[derive(Debug, Default)]
pub struct BasicGraph {
    nodes: HashMap<NodeId, String>,
    edges: HashMap<EdgeId, [NodeId; 2]>,
}

impl Graph for BasicGraph {
    fn node_ids(&self) -> Box<dyn Iterator<Item = NodeId> + '_> {
        Box::new(self.nodes.keys().copied())
    }

    fn edge_ids(&self) -> Box<dyn Iterator<Item = EdgeId> + '_> {
        Box::new(self.edges.keys().copied())
    }

    fn incident_nodes(&self, edge_id: &EdgeId) -> Option<Box<dyn Iterator<Item = NodeId> + '_>> {
        Some(Box::new(self.edges.get(edge_id)?.iter().copied()))
    }
}

#[cfg(feature = "reflect")]
impl ReflectNodes for BasicGraph {
    fn inspect_node(&mut self, id: &NodeId) -> Option<&mut dyn bevy_reflect::PartialReflect> {
        Some(self.nodes.get_mut(id)?)
    }

    fn default_node(&self) -> Box<dyn bevy_reflect::Reflect> {
        todo!()
    }

    fn insert_node(
        &mut self,
        value: Box<dyn bevy_reflect::Reflect>,
    ) -> Result<NodeId, super::error::InsertError> {
        todo!()
    }

    fn validate_node(
        &self,
        value: &dyn bevy_reflect::Reflect,
    ) -> Result<(), super::error::InsertError> {
        todo!()
    }
}

#[cfg(feature = "reflect")]
impl ReflectEdges for BasicGraph {
    fn inspect_edge(&mut self, _: &EdgeId) -> Option<&mut dyn bevy_reflect::PartialReflect> {
        None
    }

    fn default_edge(&self) -> Box<dyn bevy_reflect::Reflect> {
        todo!()
    }

    fn insert_edge(
        &mut self,
        value: Box<dyn bevy_reflect::Reflect>,
    ) -> Result<EdgeId, super::error::InsertError> {
        todo!()
    }

    fn validate_edge(
        &self,
        value: &dyn bevy_reflect::Reflect,
    ) -> Result<(), super::error::InsertError> {
        todo!()
    }
}
