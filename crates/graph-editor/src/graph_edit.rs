use std::iter::empty;

use nodus_graph::{
    graph::{Graph, ReflectableGraph},
    sets::{edge::EdgeId, vertex::NodeId},
};

use crate::visual::GraphVisual;

pub mod edge;
pub mod inspect;
pub mod node;

#[derive(Default)]
pub struct GraphEdit {
    graph: Option<Box<dyn ReflectableGraph>>,
    visual_graph: GraphVisual,
}

/// Ui based impls
impl GraphEdit {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let reponse = self.visual_graph.ui(ui);
    }
}
