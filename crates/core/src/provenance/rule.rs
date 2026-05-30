#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct RuleId(usize);

pub trait Rule {
    const ID: RuleId;
}
