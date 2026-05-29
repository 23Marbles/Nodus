mod derision;
pub mod fact_list;

#[cfg(test)]
mod test {
    use crate::{
        repr::{
            fact::fact_list::Fact,
            id::{Id, PointId, Segment},
        },
        storage::fact_handler::{FactHandler, test::TestFactHandler},
    };

    fn create_fact_handler<const N: usize>(facts: [Fact; N]) -> TestFactHandler {
        let mut handler = TestFactHandler::default();
        handler.add_facts(facts);
        handler
    }

    #[test]
    fn test_midpoint_derision() {
        let points = [PointId(Id(1)), PointId(Id(2)), PointId(Id(3))];
        let fact_handler = create_fact_handler([
            Fact::colinear(points[0].clone(), points[1].clone(), points[2].clone()),
            Fact::eq_len(
                Segment::from_points(points[0].clone(), points[1].clone()),
                Segment::from_points(points[1].clone(), points[2].clone()),
            ),
        ]);

        let derived = fact_handler.find_derived_facts();

        assert_eq!(
            derived.len(),
            1,
            "Failed with underlying facts `{:?}`",
            derived
        );
        assert_eq!(
            derived[0],
            Fact::midpoint(
                points[1].clone(),
                Segment::from_points(points[0].clone(), points[2].clone())
            )
        )
    }
}
