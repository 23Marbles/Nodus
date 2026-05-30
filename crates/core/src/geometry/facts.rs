use self::{colinear::Colinear, equal_length::EqualLength, midpoint::MidPoint};

pub mod colinear;
pub mod equal_length;
pub mod midpoint;

pub struct FactId(usize);

pub enum Fact {
    Colinear(Colinear),
    Midpoint(MidPoint),
    EqualLength(EqualLength),
}
