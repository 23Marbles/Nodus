mod derision;
pub mod fact_list;

#[cfg(test)]
mod test {
    use crate::{
        repr::fact::fact_list::Fact,
        storage::fact_handler::{FactHandler, test::TestFactHandler},
    };

    fn create_fact_handler(facts: &[Fact]) -> TestFactHandler {
        let mut handler = TestFactHandler::default();
        handler.add_facts(facts);
        handler
    }

    fn test_
}
