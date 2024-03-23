use fuzzy_logic_rs::{
    membership_functions::{self as mf, Gaussian, MembershipFunction as MF, Triangle},
    variables::{self, Variables},
};

fn main() {
    let mut v1 = variables::Variables::new_input("speed".to_string());
    v1.add_membership(MF::new(
        "L".to_string(),
        mf::Kind::Gaussian(Gaussian::new(40.0, 30.0)),
    ));
    v1.add_membership(MF::new(
        "M".to_string(),
        mf::Kind::Gaussian(Gaussian::new(70.0, 40.0)),
    ));
    let k1 = v1.fuzzify("L".to_string(), 30.0);

    let mut v2 = Variables::new_input("Distance".to_string());
    v2.add_membership(MF::new(
        "L".to_string(),
        mf::Kind::Triangle(Triangle::new(0.0, 10.0, 20.0)),
    ));
    v2.add_membership(MF::new(
        "M".to_string(),
        mf::Kind::Triangle(Triangle::new(10.0, 20.0, 30.0)),
    ));
    let k2 = v2.fuzzify("L".to_string(), 35.0);

    println!("{k1} {k2} ");
}
