#[derive(Debug)]
pub enum Aggregations {
    Max,
    Sum,
}

impl Aggregations {
    pub fn aggregation(&self, implication_vec: Vec<Vec<f64>>) -> Vec<f64> {
        match self {
            Self::Max => max_aggregation(implication_vec),
            Self::Sum => sum_aggregation(implication_vec),
        }
    }
}

pub fn max_aggregation(implication_vec: Vec<Vec<f64>>) -> Vec<f64> {
    let mut mu = Vec::new();
    let size_implication = implication_vec.len();
    let size_vec = implication_vec[0].len();
    for i in 0..size_vec {
        let mut max = implication_vec[0][i];
        for ii in 1..size_implication {
            if max < implication_vec[ii][i] {
                max = implication_vec[ii][i];
            }
        }
        mu.push(max)
    }
    mu
}

pub fn sum_aggregation(implication_vec: Vec<Vec<f64>>) -> Vec<f64> {
    let mut mu = Vec::new();
    let size_implication = implication_vec.len();
    let size_vec = implication_vec[0].len();
    for i in 0..size_implication {
        let mut sum = 0.0;
        for ii in 1..size_vec {
            sum += implication_vec[i][ii]
        }
        mu.push(sum)
    }
    mu
}
