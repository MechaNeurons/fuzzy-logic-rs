#[derive(Debug)]
pub enum SNorms {
    Max,
    Custom(Custom),
}

impl SNorms {
    pub fn s_norm(&self, fuzzified: Vec<f64>) -> f64 {
        match self {
            Self::Max => max(fuzzified),
            Self::Custom(c) => (c.func)(fuzzified),
        }
    }
}

fn max(fuzzified: Vec<f64>) -> f64 {
    assert_ne!(fuzzified.len(), 0);
    fuzzified
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
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
