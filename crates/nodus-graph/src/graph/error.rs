use std::{fmt::Display, ops::Range};

#[cfg(feature = "reflect")]
use crate::sets::{edge::EdgeId, vertex::NodeId};

#[cfg(feature = "reflect")]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InsertError {
    #[error("Expected type `{expected}`, got type `{got}`")]
    WrongType { expected: String, got: String },
    #[error("Validation errors (count = {len})", len = .0.len())]
    Validation(Vec<ValidationError>),
    #[error("Endpoints quantity should be {expected}, was `{got}`")]
    IncorrectEndpointCount { expected: EndpointRange, got: usize },
    #[error("Node with id `{id}` dose not exist")]
    NonExistentNode { id: NodeId },
    #[error("Edge with id `{id}` dose not exist")]
    NonExistentEdge { id: EdgeId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndpointRange {
    SingleValue(usize),
    Min(usize),
    Max(usize),
    Range(Range<usize>),
}

impl Display for EndpointRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndpointRange::SingleValue(v) => write!(f, "{v}"),
            EndpointRange::Min(v) => write!(f, "at least {v}"),
            EndpointRange::Max(v) => write!(f, "no more than {v}"),
            EndpointRange::Range(range) => write!(f, "between {} and {}", range.start, range.end),
        }
    }
}

#[cfg(feature = "reflect")]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Invalid state{op_field} with message `{message}`", op_field = .field.as_ref().map_or("".to_string(), |name| format!(" at field `{name}`,")))]
pub struct ValidationError {
    pub field: Option<String>,
    pub message: String,
}
