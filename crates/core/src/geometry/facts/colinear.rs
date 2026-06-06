use crate::geometry::point::PointId;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Colinear {
    pub a: PointId,
    pub b: PointId,
    pub c: PointId,
}
