#[derive(Debug)]
pub enum TNorms {
    Min,
    Product,
    Custom(fn(&[f64]) -> f64),
}

impl TNorms {
    pub fn t_norm(&self, fuzzified: &[f64]) -> f64 {
        match self {
            Self::Min => min(fuzzified),
            Self::Product => product(fuzzified),
            Self::Custom(c) => c(fuzzified),
            // _ => 0.0,
        }
    }
}

fn min(fuzzified: &[f64]) -> f64 {
    assert_ne!(fuzzified.len(), 0);
    fuzzified
        .into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        .to_owned()
}

fn product(fuzzified: &[f64]) -> f64 {
    assert_ne!(fuzzified.len(), 0);
    fuzzified.iter().product()
}
