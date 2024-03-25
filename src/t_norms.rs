#[derive(Debug)]
pub enum TNorms {
    Min,
    Product,
    Custom(Custom),
}

impl TNorms {
    pub fn t_norm(&self, fuzzified: Vec<f64>) -> f64 {
        match self {
            Self::Min => min(fuzzified),
            Self::Product => product(fuzzified),
            Self::Custom(c) => (c.func)(fuzzified),
            // _ => 0.0,
        }
    }
}

fn min(fuzzified: Vec<f64>) -> f64 {
    assert_ne!(fuzzified.len(), 0);
    fuzzified
        .into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn product(fuzzified: Vec<f64>) -> f64 {
    assert_ne!(fuzzified.len(), 0);
    fuzzified.iter().product()
}

#[derive(Debug)]
pub struct Custom {
    func: fn(Vec<f64>) -> f64,
}

impl Custom {
    pub fn new(func: fn(Vec<f64>) -> f64) -> Self {
        Self { func }
    }
}
