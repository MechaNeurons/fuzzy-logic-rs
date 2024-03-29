#[derive(Debug)]
pub enum SNorms {
    Max,
    Custom(fn(&[f64]) -> f64),
}

impl SNorms {
    pub fn s_norm(&self, fuzzified: &[f64]) -> f64 {
        match self {
            Self::Max => max(fuzzified),
            Self::Custom(c) => c(fuzzified),
        }
    }
}

fn max(fuzzified: &[f64]) -> f64 {
    assert_ne!(fuzzified.len(), 0);
    fuzzified
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        .to_owned()
}
