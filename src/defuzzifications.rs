#[derive(Debug)]
pub enum Defuzzifiers {
    Centroid,
    Bisection,
}
impl Defuzzifiers {
    pub fn defuzzify(&self, vec: Vec<f64>) -> i32 {
        match self {
            Self::Centroid => centroid_defuzzification(vec),
            Self::Bisection => bisection_defuzzification(vec),
        }
    }
}

pub fn centroid_defuzzification(vec: Vec<f64>) -> i32 {
    0
}

pub fn bisection_defuzzification(vec: Vec<f64>) -> i32 {
    let total_area: f64 = vec.iter().sum();
    let mut idx = 0;
    for i in 0..vec.len() {
        let mut area = 0.0;
        for ii in 0..i {
            area += vec[ii]
        }
        if area > total_area * 0.5 {
            idx = i as i32;
            println!("{area} {total_area}");
            break;
        }
    }
    idx
}
