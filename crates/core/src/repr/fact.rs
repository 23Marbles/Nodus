use crate::repr::id::{EndpointInersection, PointId, Segment};

#[derive(Debug, Clone, PartialEq)]
pub enum Fact {
    Midpoint(PointId, Segment),
    EqualLength(Segment, Segment),
    Colinear(PointId, PointId, PointId),
}

impl Fact {
    pub fn derive_facts(&self) -> Vec<Fact> {
        match self {
            Fact::Midpoint(point, segment) => {
                let (a, b, c) = (&segment.a, point, &segment.b);

                if b == a || b == c || segment.is_degenerate() {
                    todo!("Degenerate midpoint handling");
                }

                vec![
                    Fact::EqualLength(
                        Segment::from_points(a.clone(), b.clone()),
                        Segment::from_points(b.clone(), c.clone()),
                    ),
                    Fact::Colinear(a.clone(), b.clone(), c.clone()),
                ]
            }
            Fact::EqualLength(segment, segment1) => {
                let _ = (segment, segment1);
                Vec::with_capacity(0)
            }
            Fact::Colinear(point, point1, point2) => {
                let _ = (point, point1, point2);
                Vec::with_capacity(0)
            }
        }
    }
}

pub struct MultiFact {
    facts: Vec<Fact>,
}

impl MultiFact {
    pub fn derive_facts(&self) -> Vec<Self> {}

    fn from_eq_len_colinear(&self, colinear_idx: usize, eq_len_idx: usize) -> Option<Self> {
        let Fact::Colinear(col_a, col_b, col_c) = &self.facts.get(colinear_idx)? else {
            return None;
        };
        let Fact::EqualLength(eq_len1, eq_len2) = &self.facts.get(eq_len_idx)? else {
            return None;
        };

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
            Some(Self {
                facts: vec![Fact::Midpoint(
                    center.clone(),
                    Segment::from_points(end1.clone(), end2.clone()),
                )],
            })
        }
    }

    fn from_midpoint(&self, midpoint_idx: usize) -> Option<Self> {
        let midpoint = &self.facts.get(midpoint_idx)?;

        if !matches!(midpoint, Fact::Midpoint(_, _)) {
            return None;
        }

        Some(Self {
            facts: midpoint.derive_facts(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{repr::fact::Fact, storage::fact_handler::test::TestFactHandler};

    fn create_fact_handler(facts: &[Fact]) -> TestFactHandler {
        let mut handler = TestFactHandler::default();
    }
}
