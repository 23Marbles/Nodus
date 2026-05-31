# Fact Storage

Stores all the facts that are currently in use.

# What it has to do

- Store facts
- Have a method to add facts
- Have multiple ways of getting facts
- Have a way of getting all facts of a certain type

# Visualization

```kale
-d
A,B,C
SegmentAB
SegmentBC
SegmentAC
Midpoint
(A,SegmentAB), (B,SegmentAB)
(B,SegmentBC), (C,SegmentBC)
(A,SegmentAC), (C,SegmentAC)
(SegmentAC, Midpoint)
(B, Midpoint)
Colinear
EqLen
(A, Colinear), (B, Colinear), (C, Colinear)
(Midpoint, Colinear)
(SegmentAB, EqLen), (SegmentBC, EqLen)
(Midpoint, EqLen)
```

# Pseudocode

```rust
enum FactOrigin {
	/// User inputted fact
	Given,
	/// Derived from other facts
	Derived(DerivedFrom),
	/// Temporary constraints applied to the original points to help prove theorums
	Assumed,
	/// The program decided to insert this on its _own_ points
	Arbitrary,
}

struct FactEntry {
	id: FactId,
	/// The underlying fact
	fact: Fact,
	/// The origin
	origin: FactOrigin,
}

trait FactStorage {
	fn add_fact(&mut self, fact: F);
	fn get_fact(&self, fact_id: FactId) -> &Fact;
	fn facts(&self) -> impl Iterator<Item = &FactEntry>;
	fn get_facts_filter<P>(&self, predicate: P) -> impl Iterator<Item = &FactEntry>
	where
		P: Fn(&FactEntry) -> bool;
	fn facts_matches(&self, pat: ?) -> Vec<&::FactKind>
}
```