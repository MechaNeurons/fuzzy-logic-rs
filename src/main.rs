use fuzzy_logic_rs::{
    fuzzy_inference_system::MamdaniFIS,
    membership_functions::{Gaussian, MFKind, Triangle, MF},
    s_norms::SNorms,
    t_norms::TNorms,
    variables::Variables,
};

fn main() {
    let mut v1 = Variables::new_input("speed".to_string());
    v1.add_membership(MF::new(
        "L".to_string(),
        MFKind::Gaussian(Gaussian::new(40.0, 30.0)),
    ));
    v1.add_membership(MF::new(
        "M".to_string(),
        MFKind::Gaussian(Gaussian::new(70.0, 40.0)),
    ));

    let mut v2 = Variables::new_input("Distance".to_string());
    v2.add_membership(MF::new(
        "L".to_string(),
        MFKind::Triangle(Triangle::new(0.0, 10.0, 20.0)),
    ));
    v2.add_membership(MF::new(
        "M".to_string(),
        MFKind::Triangle(Triangle::new(10.0, 20.0, 30.0)),
    ));

    let mut o1 = Variables::new_output("Acceleration".to_string());
    o1.add_membership(MF::new(
        "SL".to_string(),
        MFKind::Gaussian(Gaussian::new(-0.5, 0.1)),
    ));

    let mut fis = MamdaniFIS::new(SNorms::Max, TNorms::Min);
    fis.add_input(v1);
    fis.add_input(v2);
    fis.add_output(o1);

    fis.compute_outputs(35.0);
}
