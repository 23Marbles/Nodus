use crate::{geometry::facts::FactId, provenance::rule::RuleId};

pub struct DerivedFrom {
    facts: Vec<FactId>,
    rule: RuleId,
}
