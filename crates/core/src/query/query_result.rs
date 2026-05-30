use crate::geometry::facts::FactId;

pub enum QueryResult<T> {
    Exact(T),
    Approximate(T),
    Contradicted(Vec<FactId>),
    UnderConstrained,
}
