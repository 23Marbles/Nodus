#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg(feature = "reflect")]
use crate::graph::error::InsertError;
use crate::sets::{edge::EdgeId, vertex::NodeId};

pub mod error;
pub mod normal;

pub trait Graph {
    fn node_ids(&self) -> Box<dyn Iterator<Item = NodeId> + '_>;
    fn edge_ids(&self) -> Box<dyn Iterator<Item = EdgeId> + '_>;

    fn incident_nodes(&self, edge_id: &EdgeId) -> Option<Box<dyn Iterator<Item = NodeId> + '_>>;
}

#[cfg(feature = "reflect")]
pub trait ReflectableGraph: ReflectNodes + ReflectEdges + CreateNodes + CreateEdges {}

#[cfg(feature = "reflect")]
impl<T: ReflectEdges + ReflectNodes + CreateNodes + CreateEdges> ReflectableGraph for T {}

#[cfg(feature = "reflect")]
pub trait CreateNodes {
    fn default_node(&self) -> Box<dyn bevy_reflect::Reflect>;
    fn insert_node(&mut self, value: Box<dyn Reflect>) -> Result<NodeId, InsertError>;
    fn validate_node(&self, value: &dyn Reflect) -> Result<(), InsertError>;
}

#[cfg(feature = "reflect")]
pub trait ReflectNodes: Graph {
    fn inspect_node(&mut self, id: &NodeId) -> Option<&mut dyn bevy_reflect::PartialReflect>;
}

#[cfg(feature = "reflect")]
pub trait CreateEdges {
    fn default_edge(&self) -> Box<dyn bevy_reflect::Reflect>;
    fn insert_edge(&mut self, value: Box<dyn Reflect>) -> Result<EdgeId, InsertError>;
    fn validate_edge(&self, value: &dyn Reflect) -> Result<(), InsertError>;
}

#[cfg(feature = "reflect")]
pub trait ReflectEdges: Graph {
    fn inspect_edge_metadata(
        &mut self,
        id: &EdgeId,
    ) -> Option<&mut dyn bevy_reflect::PartialReflect>;
    fn edge_endpoints(&self, id: &EdgeId) -> Option<Box<dyn Iterator<Item = NodeId> + '_>>;
    fn set_edge_endpoints(
        &mut self,
        id: &EdgeId,
        new_endpoints: Box<dyn Iterator<Item = NodeId> + '_>,
    ) -> Result<(), InsertError>;
}
