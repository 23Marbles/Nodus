use bevy_reflect::{PartialReflect, Reflect};

use crate::visual::edge::EdgeVisual;

pub struct GraphEdge {
    pub visual_data: EdgeVisual,
    pub underlying_data: Box<dyn Reflect>,
}

pub struct EdgeInspector<'a> {
    pub visual_data: &'a mut EdgeVisual,
    pub underlying_data: &'a mut dyn PartialReflect,
}
