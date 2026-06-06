use crate::{
    provenance::rules::{Rule, RuleIdErasedDerivedFact},
    storage::fact_storage::FactStorage,
};

pub struct MidpointFromColinearEqLen;

impl Rule for MidpointFromColinearEqLen {
    fn apply_at(&self, fact_store: FactStorage) -> Vec<RuleIdErasedDerivedFact> {}
}
