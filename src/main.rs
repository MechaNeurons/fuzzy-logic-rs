use fuzzy_logic_rs::{
    fuzzy_inference_systems::MamdaniFIS,
    membership_functions::{Gaussian, MFKind, Triangle, MF},
    rules::Rule,
    s_norms::SNorms,
    t_norms::TNorms,
    variables::Variables,
};

fn main() {
    let mut v1 = Variables::new_input("speed".to_string());
    v1.add_membership(MF::new(
        "S".to_string(),
        MFKind::Triangle(Triangle::new(-58.3, 0.0, 58.3)),
    ));
    v1.add_membership(MF::new(
        "M".to_string(),
        MFKind::Triangle(Triangle::new(11.67, 70.0, 128.3)),
    ));
    v1.add_membership(MF::new(
        "L".to_string(),
        MFKind::Triangle(Triangle::new(81.67, 140.0, 198.3)),
    ));

    let mut v2 = Variables::new_input("Distance".to_string());
    v2.add_membership(MF::new(
        "S".to_string(),
        MFKind::Triangle(Triangle::new(-20.83, 0.0, 20.83)),
    ));
    v2.add_membership(MF::new(
        "M".to_string(),
        MFKind::Triangle(Triangle::new(4.168, 25.0, 45.82)),
    ));
    v2.add_membership(MF::new(
        "L".to_string(),
        MFKind::Triangle(Triangle::new(29.17, 50.0, 70.82)),
    ));

    let mut o1 = Variables::new_output("Acceleration".to_string());
    o1.add_membership(MF::new(
        "NB".to_string(),
        MFKind::Gaussian(Gaussian::new(-1.0, 0.2123)),
    ));
    o1.add_membership(MF::new(
        "NS".to_string(),
        MFKind::Gaussian(Gaussian::new(-0.25, 0.2123)),
    ));
    o1.add_membership(MF::new(
        "ZR".to_string(),
        MFKind::Gaussian(Gaussian::new(0.0, 0.2123)),
    ));
    o1.add_membership(MF::new(
        "PS".to_string(),
        MFKind::Gaussian(Gaussian::new(0.25, 0.2123)),
    ));
    o1.add_membership(MF::new(
        "PB".to_string(),
        MFKind::Gaussian(Gaussian::new(0.8, 0.2123)),
    ));

    let mut fis = MamdaniFIS::new(SNorms::Max, TNorms::Min);
    fis.add_input(v1);
    fis.add_input(v2);
    fis.add_output(o1);

    fis.add_rule(Rule::new_and(vec![0, 0, 2], 1.0));
    fis.add_rule(Rule::new_and(vec![0, 1, 3], 1.0));
    fis.add_rule(Rule::new_and(vec![0, 2, 4], 1.0));

    fis.add_rule(Rule::new_and(vec![1, 0, 1], 1.0));
    fis.add_rule(Rule::new_and(vec![1, 1, 2], 1.0));
    fis.add_rule(Rule::new_and(vec![1, 2, 3], 1.0));

    fis.add_rule(Rule::new_and(vec![2, 0, 0], 1.0));
    fis.add_rule(Rule::new_and(vec![2, 1, 1], 1.0));
    fis.add_rule(Rule::new_and(vec![2, 2, 2], 1.0));

    fis.compute_outputs(vec![40.0, 13.0]);
}
