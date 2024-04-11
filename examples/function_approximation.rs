#[allow(unused)]
use fuzzy_logic_rs::{
    membership_functions::Gaussian,
    membership_functions::{MFKind, MembershipFunction},
    rules::TSKRule,
    variables::InputVariables,
};

#[allow(unused)]
fn main() {
    let x1 = 0.0;
    let x2 = 0.25;
    let x3 = 0.5;
    let x4 = 0.75;
    let x5 = 1.0;
    let original_function = |x| x * (1.0 - x);
    let y15 = original_function(x1);
    let y24 = original_function(x2);
    let y3 = original_function(x3);

    let mut x = InputVariables::new("X".to_string(), (0.0, 1.0));
    x.add_membership(MembershipFunction::new(
        "x1".to_string(),
        MFKind::Gaussian(Gaussian::new(0.0, 0.09)),
    ));
    x.add_membership(MembershipFunction::new(
        "x2".to_string(),
        MFKind::Gaussian(Gaussian::new(0.25, 0.09)),
    ));
    x.add_membership(MembershipFunction::new(
        "x3".to_string(),
        MFKind::Gaussian(Gaussian::new(0.5, 0.09)),
    ));
    x.add_membership(MembershipFunction::new(
        "x4".to_string(),
        MFKind::Gaussian(Gaussian::new(0.75, 0.09)),
    ));
    x.add_membership(MembershipFunction::new(
        "x5".to_string(),
        MFKind::Gaussian(Gaussian::new(1.0, 0.09)),
    ));
}
