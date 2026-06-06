use crate::geometry::{point::PointId, segment::Segment};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Midpoint {
    pub center: PointId,
    pub segment: Segment,
}
