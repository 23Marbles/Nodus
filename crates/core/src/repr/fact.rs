use crate::repr::id::{EndpointInersection, PointId, Segment};

#[derive(Debug, Clone, PartialEq)]
struct MidPoint {
    point: PointId,
    segment: Segment,
}

#[derive(Debug, Clone, PartialEq)]
struct EqualLength {
    segment1: Segment,
    segment2: Segment,
}

#[derive(Debug, Clone, PartialEq)]
struct Colinear {
    point1: PointId,
    point2: PointId,
    point3: PointId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Fact {
    Midpoint(MidPoint),
    EqualLength(EqualLength),
    Colinear(Colinear),
}

impl Fact {
    pub fn derive_facts(&self) -> Vec<Self> {
        match self {
            Fact::Midpoint(MidPoint { point, segment }) => {
                let (a, b, c) = (&segment.a, point, &segment.b);

                if b == a || b == c || segment.is_degenerate() {
                    todo!("Degenerate midpoint handling");
                }

                vec![
                    Fact::EqualLength(EqualLength {
                        segment1: Segment::from_points(a.clone(), b.clone()),
                        segment2: Segment::from_points(b.clone(), c.clone()),
                    }),
                    Fact::Colinear(Colinear {
                        point1: a.clone(),
                        point2: b.clone(),
                        point3: c.clone(),
                    }),
                ]
            }
            Fact::EqualLength(EqualLength { segment1, segment2 }) => {
                let _ = (segment1, segment2);
                Vec::new()
            }
            Fact::Colinear(Colinear {
                point1,
                point2,
                point3,
            }) => {
                let _ = (point1, point2, point3);
                Vec::new()
            }
        }
    }

    fn binary_derivision(&self, other: &Fact) -> Option<Self> {
        match (self, other) {
            (Fact::EqualLength(eq_len), Fact::Colinear(colinear))
            | (Fact::Colinear(colinear), Fact::EqualLength(eq_len)) => {
                from_colinear_eq_len(colinear, eq_len)
            }
            _ => todo!(),
        }
    }
}

fn from_colinear_eq_len(colinear: &Colinear, eq_len: &EqualLength) -> Option<Fact> {
    let Colinear { point1: col_a, point2: col_b, point3: col_c }
    let EqualLength { segment1: eq_len1, segment2: eq_len2 } = eq_len;

    let EndpointInersection { overlap, distinct } = eq_len1.endpoint_intersection(eq_len2);

    let center = if overlap.len() == 1 {
        overlap[0]
    } else {
        return None;
    };

    let (end1, end2) = if distinct.len() == 2 {
        (distinct[0], distinct[1])
    } else {
        return None;
    };

    // TODO: make this more efficient
    let points = [col_a, col_b, col_c];
    let mut hit = [false; 3];

    for (p, hit) in points.into_iter().zip(hit.iter_mut()) {
        *hit = p == center || p == end1 || p == end2;
    }

    if hit.into_iter().any(|b| !b) {
        None
    } else {
        Some(Fact::Midpoint(
            center.clone(),
            Segment::from_points(end1.clone(), end2.clone()),
        ))
    }
}

pub struct MultiFact {
    facts: Vec<Fact>,
}

impl MultiFact {
    pub fn derive_facts(&self) -> Vec<Self> {}
}

#[cfg(test)]
mod test {
    use crate::{repr::fact::Fact, storage::fact_handler::test::TestFactHandler};

    fn create_fact_handler(facts: &[Fact]) -> TestFactHandler {
        let mut handler = TestFactHandler::default();
    }
}
