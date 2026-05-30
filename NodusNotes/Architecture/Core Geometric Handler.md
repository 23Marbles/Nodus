# Geometric Handler
This acts as entry point and storage for geometric positions and nodes.

# What it has to do
- Store facts
- Answer queries based on these facts
- Give a path from _given_ facts to queried data allowing the data to be computed
- Handle impossible states elegantly

Pseudocode:
```rust
enum QueryResult<T> {
	Exact(T),
	Approximate(T),
	Contradicted(Vec<FactId>),
	UnderConstrained,
}

struct DerivedFrom {
	facts: Vec<FactId>,
	link: RuleId,
}

enum Transformation {
	/// A fact that must be true based on its underlying facts was added
	DeriveFact {
		justification: DerivedFrom,
		fact: FactId
	},
	/// A fact was added to give extra context
	AddFact(FactId),
	/// A point was added to give answers
	AddPoint(PointId),
	/// Signals the points were joined to make way for new facts
	/// Shapes are simply references to grouped points
	JoinPoints {
		points: Vec<PointId>,
		join_shape: ShapeId,
	}
}

struct HistoricalTransformation {
	depends_on: Vec<HistoryId>,
	transformation: Transformation,
}

struct TransformHistory {
	hist: HashMap<HistoryId, HistoricalTransformation>
}

enum FactOrigin {
	/// User inputted fact
	Given,
	/// Derived from other facts
	Derived,
	/// Temporary constraints applied to the original points to help prove theorums
	Assumed,
	/// The program decided to insert this on its _own_ points
	Arbitrary,
}

struct FactSource {
	origin: FactOrigin,
	proved_by: Option<DerivedFrom>
}

trait GeoHandler {
	/// Returns the history of all transformations applied to this graph since
	/// last transformation reset
	/// Used for debugging
	fn get_transformation_history(&self) -> TransformHistory;
	/// Determine the answer to a given query
	fn query(&self, query: Query) -> QueryResult<T>;
	fn add_fact(&mut self, source: FactSource, fact: Fact);
}
```