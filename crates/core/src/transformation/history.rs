use std::collections::HashMap;

use crate::transformation::Transformation;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct HistoryId(usize);

pub struct HistoricalTransformation {
    depends_on: Vec<HistoryId>,
    transformation: Transformation,
}

pub struct TransformHistory {
    hist: HashMap<HistoryId, HistoricalTransformation>,
}
