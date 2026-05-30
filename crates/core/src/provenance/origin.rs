use crate::provenance::justification::DerivedFrom;

pub enum FactOrigin {
    /// User inputted fact
    Given,
    /// Derived from other facts
    Derived(DerivedFrom),
    /// Temporary constraints applied to the original points to help prove theorums
    Assumed,
    /// The program decided to insert this on its _own_ points
    Arbitrary,
}
