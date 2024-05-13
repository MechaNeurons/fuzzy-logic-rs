use fuzzy_logic_rs::{
    defuzzifications::TSKDefuzzifiers, fuzzy_inference_systems::TSKFIS, implications::Implications, membership_functions::{Gaussian, MFKind, MembershipFunction}, rules::Rule, s_norms::SNorms, t_norms::TNorms, variables::{InputVariable, TSKOutputVariable}
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

    let mut fis = TSKFIS::new(SNorms::Max, TNorms::Min, Implications::Product,TSKDefuzzifiers::Mean);

    let mut x: InputVariable = InputVariable::new("X".to_string(), (0.0, 1.0));
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
    fis.add_input(x);

    let mut y: TSKOutputVariable = TSKOutputVariable::new("Y".to_string());
    y.add_constant_membership(y15);
    y.add_constant_membership(y24);
    y.add_constant_membership(y3);

    fis.add_output(y);

    fis.add_rule(Rule::new_and(vec![0, 0], 1.0));
    fis.add_rule(Rule::new_and(vec![1, 1], 1.0));
    fis.add_rule(Rule::new_and(vec![2, 2], 1.0));
    fis.add_rule(Rule::new_and(vec![3, 1], 1.0));
    fis.add_rule(Rule::new_and(vec![4, 0], 1.0));

    let out: Vec<f64> = fis.compute_outputs(vec![0.6]);
    println!("{:?}", out);
}
