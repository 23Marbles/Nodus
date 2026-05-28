use crate::repr::id::{PointId, Segment};

#[derive(Debug, Clone, PartialEq)]
pub struct MidPoint {
    pub point: PointId,
    pub segment: Segment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EqualLength {
    pub segment1: Segment,
    pub segment2: Segment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Colinear {
    pub point1: PointId,
    pub point2: PointId,
    pub point3: PointId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Fact {
    Midpoint(MidPoint),
    EqualLength(EqualLength),
    Colinear(Colinear),
}

impl Fact {
    pub fn midpoint(center: PointId, segment: Segment) -> Self {
        Self::Midpoint(MidPoint {
            point: center,
            segment,
        })
    }
}
