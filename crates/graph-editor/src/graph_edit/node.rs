use bevy_reflect::{PartialReflect, Reflect};
use egui::{Response, Slider, Widget};

use crate::visual::node::NodeVisual;

pub struct GraphNode {
    pub visual_data: NodeVisual,
    pub underlying_data: Box<dyn Reflect>,
}

impl GraphNode {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let inspector = NodeInspector {
            visual_data: &mut self.visual_data,
            underlying_data: self.underlying_data.as_mut(),
        };
        inspector.ui(ui);
    }
}

pub struct NodeInspector<'a> {
    pub visual_data: &'a mut NodeVisual,
    pub underlying_data: &'a mut dyn PartialReflect,
}

impl Widget for NodeInspector<'_> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let response1 = ui.label("Node inspector");
        let response2 =
            ui.add(Slider::new(&mut self.visual_data.radius, 2.0..=30.0).text("Radius"));

        response1.union(response2)
    }
}
