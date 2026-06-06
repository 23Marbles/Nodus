use crate::geometry::point::PointId;

#[derive(Debug, Clone, Eq)]
pub struct Segment {
    pub a: PointId,
    pub b: PointId,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.standardised() == other.standardised()
    }
}

impl Segment {
    /// Gives a standardised version of points, with the greatest PointId being first
    fn standardised(&self) -> (PointId, PointId) {
        if self.a > self.b {
            (self.a, self.b)
        } else {
            (self.b, self.a)
        }
    }
}
