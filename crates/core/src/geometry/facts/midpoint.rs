use crate::geometry::{point::PointId, segment::Segment};

pub struct MidPoint {
    pub center: PointId,
    pub segment: Segment,
}
