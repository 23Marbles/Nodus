use crate::{
    geometry::facts::Fact,
    provenance::origin::FactOrigin,
    query::{query::Query, query_result::QueryResult},
    transformation::history::TransformHistory,
};

pub trait GeoHandler {
    /// Returns the history of all transformations applied to this graph since
    /// last transformation reset
    /// Used for debugging
    fn get_transformation_history(&self) -> TransformHistory;
    /// Determine the answer to a given query
    fn query<Q: Query>(&self, query: Q) -> QueryResult<Q::Output>;
    fn add_fact(&mut self, origin: FactOrigin, fact: Fact);
}
