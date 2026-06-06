#[derive(Debug, Clone)]
pub struct EdgeVisual {
    thickness: f32,
}

impl Default for EdgeVisual {
    fn default() -> Self {
        Self { thickness: 3.0 }
    }
}
