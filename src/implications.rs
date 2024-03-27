#[derive(Debug)]
pub enum Implications {
    Min,
    Product,
    Custom(fn(f64, &Vec<f64>) -> Vec<f64>),
}

impl Implications {
    pub fn implication(&self, mu: f64, vec: &Vec<f64>) -> Vec<f64> {
        match self {
            Self::Min => min_implication(mu, &vec),
            Self::Product => product_implication(mu, &vec),
            Self::Custom(func) => func(mu, &vec),
        }
    }
}

pub fn min_implication(mu: f64, vec: &Vec<f64>) -> Vec<f64> {
    vec.iter().map(|e| e.min(mu)).collect()
}

pub fn product_implication(mu: f64, vec: &Vec<f64>) -> Vec<f64> {
    vec.iter().map(|e| e * mu).collect()
}
