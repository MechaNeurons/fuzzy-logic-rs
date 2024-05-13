use fuzzy_logic_rs::{
    defuzzifications::TSKDefuzzifiers,
    fuzzy_inference_systems::TSKFIS,
    membership_functions::{Gaussian, MFKind, MembershipFunction},
    rules::Rule,
    s_norms::SNorms,
    t_norms::TNorms,
    variables::{InputVariable, TSKOutputVariable},
};

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

    let mut fis = TSKFIS::new(SNorms::Max, TNorms::Min, TSKDefuzzifiers::Mean);

    let mut x: InputVariable = InputVariable::new("X".to_string(), (0.0, 1.0));
    x.add_membership(MembershipFunction::new(
        "x1".to_string(),
        MFKind::Gaussian(Gaussian::new(x1, 0.09)),
    ));
    x.add_membership(MembershipFunction::new(
        "x2".to_string(),
        MFKind::Gaussian(Gaussian::new(x2, 0.09)),
    ));
    x.add_membership(MembershipFunction::new(
        "x3".to_string(),
        MFKind::Gaussian(Gaussian::new(x3, 0.09)),
    ));
    x.add_membership(MembershipFunction::new(
        "x4".to_string(),
        MFKind::Gaussian(Gaussian::new(x4, 0.09)),
    ));
    x.add_membership(MembershipFunction::new(
        "x5".to_string(),
        MFKind::Gaussian(Gaussian::new(x5, 0.09)),
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
    let v = [
        0., 0.03448276, 0.06896552, 0.10344828, 0.13793103, 0.17241379, 0.20689655, 0.24137931,
        0.27586207, 0.31034483, 0.34482759, 0.37931034, 0.4137931, 0.44827586, 0.48275862,
        0.51724138, 0.55172414, 0.5862069, 0.62068966, 0.65517241, 0.68965517, 0.72413793,
        0.75862069, 0.79310345, 0.82758621, 0.86206897, 0.89655172, 0.93103448, 0.96551724, 1.,
    ];
    
    let out: Vec<f64> = fis.compute_outputs(vec![0.6]);
    println!("{:?}", out);
}
