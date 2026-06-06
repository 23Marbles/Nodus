use egui::{Pos2, Vec2};

#[derive(Debug, Clone)]
pub struct Camera {
    pub offset: Vec2,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            offset: Default::default(),
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn graph_to_screen(&self, pos: Pos2) -> Pos2 {
        Pos2::new(
            pos.x * self.zoom + self.offset.x,
            pos.y * self.zoom + self.offset.y,
        )
    }

    pub fn screen_to_graph(&self, pos: Pos2) -> Pos2 {
        Pos2::new(
            (pos.x - self.offset.x) / self.zoom,
            (pos.y - self.offset.y) / self.zoom,
        )
    }
}
