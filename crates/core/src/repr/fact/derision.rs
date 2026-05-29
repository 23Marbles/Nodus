use crate::repr::{
    fact::fact_list::{Colinear, EqualLength, Fact, Midpoint},
    id::{EndpointInersection, Segment},
};

/// Derision handlers
impl Fact {
    pub fn derive_facts(&self) -> Vec<Self> {
        match self {
            Fact::Midpoint(Midpoint { point, segment }) => {
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
}

/// Derision helpers
impl Fact {
    pub fn from_colinear_eq_len(colinear: &Colinear, eq_len: &EqualLength) -> Option<Fact> {
        let Colinear {
            point1: col_a,
            point2: col_b,
            point3: col_c,
        } = colinear;
        let EqualLength {
            segment1: eq_len1,
            segment2: eq_len2,
        } = eq_len;

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
            Some(Fact::midpoint(
                center.clone(),
                Segment::from_points(end1.clone(), end2.clone()),
            ))
        }
    }
}
