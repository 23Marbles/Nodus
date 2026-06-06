use std::collections::HashMap;

use egui::{Color32, Sense, epaint::CircleShape};
use nodus_graph::sets::{edge::EdgeId, vertex::NodeId};

use crate::{
    context::menu::ContextMenuKind,
    visual::{edge::EdgeVisual, node::NodeVisual, view::Camera},
};

pub mod edge;
pub mod node;
pub mod view;

#[derive(Debug, Default, Clone)]
pub struct GraphVisual {
    pub camera: Camera,
    pub nodes: HashMap<NodeId, NodeVisual>,
    pub edges: HashMap<EdgeId, EdgeVisual>,
}

#[derive(Debug, Default)]
pub struct GraphVisualResponse {
    open_context_menu: Option<ContextMenuKind>,

    select_node: Option<NodeId>,
    select_edge: Option<EdgeId>,
}

impl GraphVisual {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> GraphVisualResponse {
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

        let screen_rect = response.rect;

        for (id, node_visual) in &self.nodes {
            if node_visual.is_on_screen(screen_rect) {
                painter.add(CircleShape::filled(
                    self.camera.graph_to_screen(*&node_visual.pos.into()),
                    3.0,
                    Color32::WHITE,
                ));
            }
        }

        GraphVisualResponse::default()
    }
}
