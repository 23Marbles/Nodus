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
        fn get_facts(&self) -> Box<[&Fact]> {
            self.facts.iter().collect::<Box<[&Fact]>>()
        }

        fn add_fact(&mut self, fact: Fact) {
            self.facts.push(fact);
        }

        /// THIS FUNCTION IS INCREDIBLY INNEFFICIENT
        fn find_derived_facts(&self) -> Vec<Fact> {
            fn factorial(input: usize) -> usize {
                #[cfg(target_arch = "x86")]
                if input > 12 {
                    panic!("Cannot fit factorials larger than 12! into a 32 bit usize")
                };
                #[cfg(target_arch = "x86_64")]
                if input > 20 {
                    panic!("Cannot fit factorials larger than 20! into a 64 bit usize")
                };

                if input == 0 {
                    1
                } else {
                    input * factorial(input - 1)
                }
            }

            let vectors: Vec<Vec<_>> = Vec::with_capacity(factorial(self.facts.len()));
        }
    }
}
