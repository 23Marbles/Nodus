use crate::{geometry::facts::FactId, provenance::rules::RuleId};

pub enum FactOrigin {
    /// User inputted fact
    Given,
    /// Derived from other facts
    Derived { premises: Vec<FactId>, rule: RuleId },
    /// Temporary constraints applied to the original points to help prove theorums
    Assumed,
    /// The program decided to insert this on its _own_ points
    Arbitrary,
}
