use self::{colinear::Colinear, equal_length::EqualLength, midpoint::Midpoint};

pub mod colinear;
pub mod equal_length;
pub mod midpoint;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct FactId(usize);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Fact {
    Colinear(Colinear),
    Midpoint(Midpoint),
    EqualLength(EqualLength),
}
