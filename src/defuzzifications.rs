use std::iter::zip;

#[derive(Debug)]
pub enum Defuzzifiers {
    Centroid,
    Bisection,
}
impl Defuzzifiers {
    pub fn defuzzify(&self, vec: Vec<f64>, univerers: &Vec<f64>) -> f64 {
        match self {
            Self::Centroid => centroid_defuzzification(vec, univerers),
            Self::Bisection => bisection_defuzzification(vec, univerers),
        }
    }
}

pub fn centroid_defuzzification(vec: Vec<f64>, universe: &Vec<f64>) -> f64 {
    let numerator: f64 = zip(&vec, universe).map(|(e, u)| e * u).sum();
    let denomerator: f64 = (&vec).iter().sum();
    let idx = numerator / denomerator;
    idx
}

pub fn bisection_defuzzification(vec: Vec<f64>, universe: &Vec<f64>) -> f64 {
    let total_area: f64 = vec.iter().sum();
    let mut idx = 0;
    for i in 0..vec.len() {
        let mut area = 0.0;
        for ii in 0..i {
            area += vec[ii]
        }
        if area > total_area * 0.5 {
            idx = i;
            break;
        }
    }
    universe[idx]
}
