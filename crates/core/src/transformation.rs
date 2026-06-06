use crate::geometry::{facts::FactId, point::PointId};

pub mod history;

pub enum Transformation {
    /// A fact that must be true based on its underlying facts was added
    DeriveFact(DerivedFact),
    /// A fact was added to give extra context
    AddFact(FactId),
    /// A point was added to give answers
    AddPoint(PointId),
}
