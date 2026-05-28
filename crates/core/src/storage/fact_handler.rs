use crate::repr::fact::{Fact, MultiFact};

pub trait FactHandler {
    fn get_facts(&self) -> Box<[&Fact]>;
    fn add_fact(&mut self, fact: Fact);
    fn add_facts(&mut self, facts: impl IntoIterator<Item = Fact>) {
        for fact in facts {
            self.add_fact(fact);
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TestFactHandler {
        pub facts: Vec<Fact>,
    }

    impl FactHandler for TestFactHandler {
        fn get_facts(&self) -> Box<[&Fact]> {
            self.facts.iter().collect::<Box<[&Fact]>>()
        }

        fn add_fact(&mut self, fact: Fact) {
            self.facts.push(fact);
        }
    }
}
