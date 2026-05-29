use crate::repr::fact::fact_list::Fact;

pub trait FactHandler {
    fn get_facts(&self) -> &[Fact];
    fn add_fact(&mut self, fact: Fact);
    fn add_facts(&mut self, facts: impl IntoIterator<Item = Fact>) {
        for fact in facts {
            self.add_fact(fact);
        }
    }
    fn find_derived_facts(&self) -> Vec<Fact>;
    fn add_derived_facts(&mut self) {
        let facts = self.find_derived_facts().into_iter();
        self.add_facts(facts);
    }
    fn run_inference_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.add_derived_facts();
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
        fn get_facts(&self) -> &[Fact] {
            &self.facts
        }

        fn add_fact(&mut self, fact: Fact) {
            self.facts.push(fact);
        }

        fn find_derived_facts(&self) -> Vec<Fact> {
            let mut new_facts = Vec::new();

            let mut colinear_facts = Vec::new();
            let mut midpoint_facts = Vec::new();
            let mut eq_len_facts = Vec::new();

            for fact in &self.facts {
                new_facts.append(&mut fact.derive_facts());

                match fact {
                    Fact::Midpoint(mid_point) => midpoint_facts.push(mid_point),
                    Fact::EqualLength(equal_length) => eq_len_facts.push(equal_length),
                    Fact::Colinear(colinear) => colinear_facts.push(colinear),
                }
            }

            for colinear in colinear_facts {
                for eq_len in eq_len_facts.iter() {
                    if let Some(derived_fact) = Fact::from_colinear_eq_len(colinear, eq_len) {
                        new_facts.push(derived_fact);
                    }
                }
            }

            new_facts
        }
    }
}
