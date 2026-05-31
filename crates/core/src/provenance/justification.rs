use crate::{geometry::facts::FactId, provenance::rules::RuleId};

pub struct DerivedFrom {
    facts: Vec<FactId>,
    rule: RuleId,
}
