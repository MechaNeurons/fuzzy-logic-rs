pub mod aggregations;
pub mod defuzzifications;
pub mod fuzzy_inference_systems;
pub mod implications;
pub mod membership_functions;
pub mod membership_ranges;
pub mod rules;
pub mod s_norms;
pub mod t_norms;
pub mod variables;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        use implications::min_implication;
        use membership_ranges::{MembershipRange, Universe};
        let u1 = Universe::new(0.0, 20.0, 50);
        let r1 = MembershipRange::new_gaussian(&u1, String::from("L"), 10.0, 4.0);

        println!("test 1");
        println!("{:#?}", min_implication(0.5, &r1.get_mu()));
    }
    #[test]
    fn test2() {
        use implications::product_implication;
        use membership_ranges::{MembershipRange, Universe};
        let u1 = Universe::new(0.0, 20.0, 50);
        let r1 = MembershipRange::new_gaussian(&u1, String::from("L"), 10.0, 4.0);

        println!("test 1");
        println!("{:#?}", product_implication(0.5, &r1.get_mu()));
    }
}
