use std::collections::HashMap;

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
mod reflect_impls {
    use std::{
        any::{TypeId, type_name, type_name_of_val},
        iter::repeat_n,
        sync::LazyLock,
    };

    use crate::{
        graph::{
            error::{EndpointRange, InsertError},
            reflect::{
                CreateEdges, CreateNodes, ReflectEdgeData, ReflectEdgeDataRef, ReflectEdges,
                ReflectNodes,
            },
        },
        sets::{
            edge::{EdgeData, EdgeIdPool},
            vertex::NodeIdPool,
        },
    };

    use super::*;

    static NODE_ID_GEN: LazyLock<NodeIdPool> = LazyLock::new(|| NodeIdPool::default());

    impl CreateNodes for BasicGraph {
        fn default_node(&self) -> Box<dyn bevy_reflect::Reflect> {
            Box::new(())
        }

        fn insert_node(
            &mut self,
            value: Box<dyn bevy_reflect::Reflect>,
        ) -> Result<NodeId, InsertError> {
            let name = *value
                .downcast::<String>()
                .map_err(|d| InsertError::WrongType {
                    expected: type_name::<String>().to_owned(),
                    got: type_name_of_val(d.as_any()).to_owned(),
                })?;

            let mut new_id = NODE_ID_GEN.alloc();

            while self.nodes.contains_key(&new_id) {
                NODE_ID_GEN.free(new_id);
                new_id = NODE_ID_GEN.alloc();
            }

            self.nodes.insert(new_id, name);

            Ok(new_id)
        }

        fn validate_node(&self, value: &dyn bevy_reflect::Reflect) -> Result<(), InsertError> {
            if value.type_id() != TypeId::of::<String>() {
                Err(InsertError::WrongType {
                    expected: type_name::<String>().to_owned(),
                    got: type_name_of_val(value.as_any()).to_owned(),
                })
            } else {
                Ok(())
            }
        }
    }

    impl ReflectNodes for BasicGraph {
        fn inspect_node(&mut self, id: &NodeId) -> Option<&mut dyn bevy_reflect::PartialReflect> {
            Some(self.nodes.get_mut(id)?)
        }
    }

    impl BasicGraph {
        fn get_endpoints(
            &self,
            endpoints: impl Iterator<Item = NodeId>,
        ) -> Result<[NodeId; 2], InsertError> {
            let mut it = endpoints;

            let end1 = it.next().ok_or(InsertError::IncorrectEndpointCount {
                expected: EndpointRange::SingleValue(2),
                got: 0,
            })?;

            let end2 = it.next().ok_or(InsertError::IncorrectEndpointCount {
                expected: EndpointRange::SingleValue(2),
                got: 1,
            })?;

            // Get remaining len
            match it.count() {
                0 => (),
                x => {
                    return Err(InsertError::IncorrectEndpointCount {
                        expected: EndpointRange::SingleValue(2),
                        got: x + 2,
                    });
                }
            }

            if !self.nodes.contains_key(&end1) {
                return Err(InsertError::NonExistentNode { id: end1 });
            }

            if !self.nodes.contains_key(&end2) {
                return Err(InsertError::NonExistentNode { id: end2 });
            }

            Ok([end1, end2])
        }
    }

    static EDGE_ID_GEN: LazyLock<EdgeIdPool> = LazyLock::new(|| EdgeIdPool::default());

    impl CreateEdges for BasicGraph {
        fn default_edge(&self) -> ReflectEdgeData {
            EdgeData {
                coinident_nodes: Box::new(repeat_n(NodeId::default(), 2).into_iter()),
                meta: Box::new(()),
            }
        }

        fn insert_edge(&mut self, value: ReflectEdgeData) -> Result<EdgeId, InsertError> {
            let endpoints = self.get_endpoints(value.coinident_nodes)?;

            // Error check
            value
                .meta
                .downcast::<()>()
                .map_err(|d| InsertError::WrongType {
                    expected: type_name::<()>().to_owned(),
                    got: type_name_of_val(d.as_any()).to_owned(),
                })?;

            let mut new_id = EDGE_ID_GEN.alloc();

            while self.edges.contains_key(&new_id) {
                EDGE_ID_GEN.free(new_id);
                new_id = EDGE_ID_GEN.alloc();
            }

            self.edges.insert(new_id, endpoints);

            Ok(new_id)
        }

        fn validate_edge(&self, value: ReflectEdgeDataRef) -> Result<(), InsertError> {
            let _ = self.get_endpoints(value.coinident_nodes)?;
            let _ = value
                .meta
                .downcast_ref::<()>()
                .ok_or(InsertError::WrongType {
                    expected: type_name::<()>().to_owned(),
                    got: type_name_of_val(value.meta.as_any()).to_string(),
                })?;

            Ok(())
        }
    }

    impl ReflectEdges for BasicGraph {
        fn inspect_edge_metadata(
            &mut self,
            _: &EdgeId,
        ) -> Option<&mut dyn bevy_reflect::PartialReflect> {
            None
        }

        fn endpoint_size(&self) -> EndpointRange {
            EndpointRange::SingleValue(2)
        }

        fn edge_endpoints(&self, id: &EdgeId) -> Option<Box<dyn Iterator<Item = NodeId> + '_>> {
            Some(Box::new(self.edges.get(id)?.clone().into_iter()))
        }

        fn set_edge_endpoints(
            &mut self,
            id: &EdgeId,
            new_endpoints: Box<dyn Iterator<Item = NodeId> + '_>,
        ) -> Result<(), InsertError> {
            let ends = self.get_endpoints(new_endpoints)?;

            let endpoints = self
                .edges
                .get_mut(id)
                .ok_or(InsertError::NonExistentEdge { id: *id })?;

            *endpoints = ends;

            Ok(())
        }
    }
}
