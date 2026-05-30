use std::collections::HashMap;

use crate::transformation::Transformation;

pub struct HistoryId(usize);

pub struct HistoricalTransformation {
    depends_on: Vec<HistoryId>,
    transformation: Transformation,
}

pub struct TransformHistory {
    hist: HashMap<HistoryId, HistoricalTransformation>,
}
