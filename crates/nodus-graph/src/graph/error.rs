#[cfg(feature = "reflect")]
#[derive(Debug, Clone, thiserror::Error)]
pub enum InsertError {
    #[error("Expected type `{expected}`, got type `{got}`")]
    WrongType { expected: Box<str>, got: Box<str> },
    #[error("Validation errors (count = {len})", len = .0.len())]
    Validation(Vec<ValidationError>),
}

#[cfg(feature = "reflect")]
#[derive(Debug, Clone, thiserror::Error)]
#[error("Invalid state{op_field} with message `{message}`", op_field = .field.as_ref().map_or("".to_string(), |name| format!(" at field `{name}`,")))]
pub struct ValidationError {
    pub field: Option<String>,
    pub message: String,
}
