use egui::{Pos2, Rect};

#[derive(Debug, Default, Clone)]
pub struct NodeVisual {
    pub(super) pos: Pos2,
    pub radius: f32,
}

impl NodeVisual {
    pub fn new(pos: Pos2) -> Self {
        Self { pos, radius: 7.0 }
    }

    pub fn is_on_screen(&self, screen_rect: Rect) -> bool {
        (screen_rect.min.x <= self.pos.x && screen_rect.min.y <= self.pos.y)
            && (self.pos.x <= screen_rect.max.x && self.pos.y <= screen_rect.max.y)
    }
}
