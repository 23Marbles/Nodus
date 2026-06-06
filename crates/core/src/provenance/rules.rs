use crate::{
    geometry::facts::{Fact, FactId},
    storage::fact_storage::FactStorage,
};

pub mod midpoint;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct RuleId(usize);

/// Used so that rules don't have to know their ID
pub struct RuleIdErasedDerivedFact {
    premises: Vec<FactId>,
    fact: Fact,
}

pub trait Rule {
    fn apply_at(&self, fact_store: FactStorage, base: ) -> Vec<RuleIdErasedDerivedFact>;
}
