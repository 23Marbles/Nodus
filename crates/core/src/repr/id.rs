use std::hash::{DefaultHasher, Hash, Hasher};

use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(transparent)]
pub struct Id(pub u64);

#[derive(Debug, PartialEq, Clone, Hash)]
#[repr(transparent)]
pub struct PointId(pub Id);

#[derive(Debug, Clone, Hash)]
pub struct Segment {
    pub a: PointId,
    pub b: PointId,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

#[derive(Debug, Default)]
pub struct EndpointInersection<'seg> {
    pub overlap: SmallVec<[&'seg PointId; 2]>,
    pub distinct: SmallVec<[&'seg PointId; 2]>,
}

impl Segment {
    /// Must hold that from_points(x, y) == from_points(y, x)
    pub fn from_points(a: PointId, b: PointId) -> Self {
        Self { a, b }
    }

    pub fn endpoint_intersection<'seg>(&'seg self, other: &'seg Self) -> EndpointInersection<'seg> {
        if self.is_degenerate() || other.is_degenerate() {
            todo!("Handle degenerate line endpoint intersections")
        }

        let mut intersection = EndpointInersection::default();

        if self.a == other.a || self.a == other.b {
            intersection.overlap.push(&self.a);
        } else {
            intersection.distinct.push(&self.a);
        }

        if self.b == other.a || self.b == other.b {
            intersection.overlap.push(&self.b);
        } else {
            intersection.distinct.push(&self.b);
        }

        intersection
    }

    /// Finds all the points that are equal
    pub fn touching_points(&self, other: &Self) -> Vec<PointId> {
        match (self.as_point(), other.as_point()) {
            (None, None) => {
                match (
                    self.a == other.a,
                    self.a == other.b,
                    self.b == other.a,
                    self.b == other.b,
                ) {
                    (_, true, _, true)
                    | (true, _, true, _)
                    | (true, true, _, _)
                    | (_, _, true, true) => unreachable!("This shape is degenerate"),
                    (true, false, false, true) | (false, true, true, false) => {
                        return vec![self.a.clone(), self.b.clone()];
                    }
                    (true, false, false, false) | (false, true, false, false) => {
                        return vec![self.a.clone()];
                    }
                    (false, false, true, false) | (false, false, false, true) => {
                        return vec![self.b.clone()];
                    }
                    (false, false, false, false) => (),
                }
            }
            (None, Some(p1)) => {
                if p1 == self.a || p1 == self.b {
                    return vec![p1];
                }
            }
            (Some(p1), None) => {
                if p1 == other.a || p1 == other.b {
                    return vec![p1];
                }
            }
            (Some(p1), Some(p2)) => {
                if p1 == p2 {
                    return vec![p1];
                }
            }
        }

        Vec::with_capacity(0)
    }

    /// Checks if the line is actually point (sometimes reffered to as a degenerate line)
    #[doc(alias = "is_point")]
    pub fn is_degenerate(&self) -> bool {
        self.a == self.b
    }

    /// If it is degenerate this will return the point that they are both at
    pub fn as_point(&self) -> Option<PointId> {
        if self.is_degenerate() {
            Some(self.a.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::repr::id::{Id, PointId, Segment};

    fn test_commutative() {
        let (a, b) = (PointId(Id(0)), PointId(Id(1)));
        assert_eq!(
            Segment::from_points(a.clone(), b.clone()),
            Segment::from_points(b, a)
        );
    }
}
