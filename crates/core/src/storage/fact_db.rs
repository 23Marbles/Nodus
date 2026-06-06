use std::collections::HashMap;

use crate::{
    geometry::facts::{Fact, FactId},
    provenance::origin::FactOrigin,
};

pub struct StoredFact {
    id: FactId,
    fact: Fact,
    origin: FactOrigin,
}

pub struct FactDB {
    facts: HashMap<FactId, Fact>,
}
