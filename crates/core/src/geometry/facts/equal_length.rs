use crate::geometry::segment::Segment;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqualLength {
    pub a: Segment,
    pub b: Segment,
}
