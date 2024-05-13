use std::iter::zip;

#[derive(Debug)]
pub enum Defuzzifiers {
    Centroid,
    Bisection,
    Custom(fn(Vec<f64>, &Vec<f64>) -> f64),
}

impl Defuzzifiers {
    pub fn defuzzify(&self, vec: Vec<f64>, universe: &Vec<f64>) -> f64 {
        match self {
            Self::Centroid => centroid_defuzzification(vec, universe),
            Self::Bisection => bisection_defuzzification(vec, universe),
            Self::Custom(f) => f(vec, universe),
        }
    }
}

pub fn centroid_defuzzification(vec: Vec<f64>, universe: &Vec<f64>) -> f64 {
    let numerator: f64 = zip(&vec, universe).map(|(e, u)| e * u).sum();
    let denominator: f64 = (&vec).iter().sum();
    let idx = numerator / denominator;
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

#[derive(Debug)]
pub enum TSKDefuzzifiers {
    Mean,
    Custom(fn(&Vec<f64>, &Vec<f64>) -> f64),
}

impl TSKDefuzzifiers {
    pub fn defuzzify(&self, mu_vec: &Vec<f64>, weighed_input: &Vec<f64>) -> f64 {
        match self {
            Self::Mean => mean_tsk_defuzzification(mu_vec, weighed_input),
            Self::Custom(fun) => fun(mu_vec, weighed_input),
        }
    }
}

pub fn mean_tsk_defuzzification(mu_vec: &Vec<f64>, weighed_input: &Vec<f64>) -> f64 {
    let den: f64 = weighed_input.iter().sum();
    let num: f64 = mu_vec.iter().zip(weighed_input).map(|(mu, i)| mu * i).sum();
    num / den
}
