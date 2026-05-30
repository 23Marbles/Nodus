use crate::{
    geometry::{facts::FactId, point::PointId, shape::ShapeId},
    provenance::justification::DerivedFrom,
};

pub mod history;

pub enum Transformation {
    /// A fact that must be true based on its underlying facts was added
    DeriveFact {
        justification: DerivedFrom,
        fact: FactId,
    },
    /// A fact was added to give extra context
    AddFact(FactId),
    /// A point was added to give answers
    AddPoint(PointId),
    /// Signals the points were joined to make way for new facts
    /// Shapes are simply references to grouped points
    /// and facts that build them up
    CreateShape(ShapeId),
}
