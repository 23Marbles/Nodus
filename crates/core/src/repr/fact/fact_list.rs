use crate::repr::id::{PointId, Segment};

#[derive(Debug, Clone, PartialEq)]
pub struct Midpoint {
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
    Midpoint(Midpoint),
    EqualLength(EqualLength),
    Colinear(Colinear),
}

macro_rules! fact_from_inner {
    ($inner:ty, $path:path) => {
        impl From<$inner> for Fact {
            fn from(value: $inner) -> Self {
                $path(value)
            }
        }
    };
}

fact_from_inner!(EqualLength, Fact::EqualLength);
fact_from_inner!(Midpoint, Fact::Midpoint);
fact_from_inner!(Colinear, Fact::Colinear);

impl Fact {
    pub fn midpoint(center: PointId, segment: Segment) -> Self {
        Self::Midpoint(Midpoint {
            point: center,
            segment,
        })
    }

    pub fn eq_len(segment1: Segment, segment2: Segment) -> Self {
        Self::EqualLength(EqualLength { segment1, segment2 })
    }

    pub fn colinear(point1: PointId, point2: PointId, point3: PointId) -> Self {
        Self::Colinear(Colinear {
            point1,
            point2,
            point3,
        })
    }
}
