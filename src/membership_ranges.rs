// use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct MembershipRange {
    name: String,
    mu: Vec<f64>,
}
/*
#[derive(Debug)]
pub enum MembershipRangeKind {
    Triangle,
    Trapezoid,
    Gaussian,
    Custom,
}
*/
impl MembershipRange {
    pub fn new(name: String, mu: Vec<f64>) -> Self {
        Self { name, mu }
    }
    pub fn new_triangle(universe: &Vec<f64>, name: String, a: f64, b: f64, c: f64) -> Self {
        assert!(a < b, "a must be less than b");
        assert!(b < c, "b must be less that c");
        let mut mu: Vec<f64> = Vec::new();
        for x in universe.into_iter() {
            let data: f64;
            if *x <= a {
                data = 0.0;
            } else if *x <= b {
                data = (*x - a) / (b - a);
            } else if *x <= c {
                data = (c - *x) / (c - b);
            } else {
                data = 0.0;
            }
            mu.push(data);
        }
        Self { name, mu }
    }
    pub fn new_trapezoid(
        universe: &Vec<f64>,
        name: String,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    ) -> Self {
        // assert!(universe[0]<a);
        assert!(a <= b, "a must be less than b");
        assert!(b <= c, "b must be less than c");
        assert!(c <= d, "c must be less than d");
        let mut mu: Vec<f64> = Vec::new();
        let mut data: f64;
        for value in universe.into_iter() {
            if *value <= a {
                data = 0.0;
            } else if *value <= b {
                data = (*value - a) / (b - a);
            } else if *value <= c {
                data = 1.0;
            } else if *value <= d {
                data = (d - *value) / (d - c);
            } else {
                data = 0.0;
            }
            mu.push(data);
        }
        Self { name, mu }
    }

    pub fn new_linearz(universe: &Vec<f64>, name: String, a: f64, b: f64) -> Self {
        assert!(a < b);
        let mut mu: Vec<f64> = Vec::new();
        let mut data: f64;
        for x in universe.into_iter() {
            if *x < a {
                data = 1.0;
            } else if *x < b {
                data = (a - *x) / (a - b);
            } else {
                data = 0.0;
            }
            mu.push(data);
        }
        Self { name, mu }
    }

    pub fn new_linears(universe: &Vec<f64>, name: String, a: f64, b: f64) -> Self {
        assert!(a < b);
        let mut mu: Vec<f64> = Vec::new();
        let mut data: f64;
        for x in universe.into_iter() {
            if *x < a {
                data = 0.0;
            } else if *x < b {
                data = (*x - a) / (b - a);
            } else {
                data = 1.0;
            }
            mu.push(data);
        }
        Self { name, mu }
    }

    pub fn new_step_down(universe: &Vec<f64>, name: String, a: f64) -> Self {
        let mut mu: Vec<f64> = Vec::new();
        for x in universe.into_iter() {
            let mut data: f64 = 1.0;
            if *x > a {
                data = 0.0;
            }
            mu.push(data);
        }
        Self { name, mu }
    }

    pub fn new_step_up(universe: &Vec<f64>, name: String, a: f64) -> Self {
        let mut mu: Vec<f64> = Vec::new();
        for x in universe.into_iter() {
            let mut data: f64 = 0.0;
            if *x > a {
                data = 1.0;
            }
            mu.push(data);
        }
        Self { name, mu }
    }

    pub fn new_gaussian(universe: &Vec<f64>, name: String, mean: f64, variance: f64) -> Self {
        let mut mu: Vec<f64> = Vec::new();
        assert!(variance > 0.0);
        for x in universe.into_iter() {
            let data: f64 = f64::exp(-0.5 * f64::powi((*x - mean) / variance, 2));
            mu.push(data);
        }
        Self { name, mu }
    }

    pub fn new_double_gaussian(
        universe: &Vec<f64>,
        name: String,
        mean1: f64,
        variance1: f64,
        mean2: f64,
        variance2: f64,
    ) -> Self {
        assert!(mean1 <= mean2, "mean1 must be less than mean2");
        assert!(variance1 > 0.0);
        assert!(variance2 > 0.0);
        let mut mu: Vec<f64> = Vec::new();
        for x in universe.into_iter() {
            let data: f64;
            if *x < mean1 {
                data = f64::exp(-0.5 * f64::powi((*x - mean1) / variance1, 2));
            } else if *x < mean2 {
                data = 1.0;
            } else {
                data = f64::exp(-0.5 * f64::powi((*x - mean2) / variance2, 2));
            }
            mu.push(data);
        }
        Self { name, mu }
    }

    pub fn new_bell(
        universe: &Vec<f64>,
        name: String,
        width: f64,
        shape: f64,
        center: f64,
    ) -> Self {
        let mut mu: Vec<f64> = Vec::new();
        assert!(width > 0.0);
        assert!(shape > 0.0);
        for x in universe.into_iter() {
            let data = 1.0 / (1.0 + f64::powf(f64::abs((*x - center) / width), 2.0 * shape));
            mu.push(data);
        }
        Self { name, mu }
    }

    pub fn get_mu(&self) -> &Vec<f64> {
        &self.mu
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl IntoIterator for MembershipRange {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<f64>;
    fn into_iter(self) -> Self::IntoIter {
        self.mu.into_iter()
    }
}

/*
impl Add for MembershipRange {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.universe as *const _, rhs.universe as *const _,
            "Universes must be the same"
        );
        let mut mu: Vec<f64> = Vec::new();
        let mut max: f64 = -f64::INFINITY;
        let epsilon: f64 = 0.05 * (self.universe[1] - self.universe[0]);

        for (i, x) in self.universe.into_iter().enumerate() {
            mu.push(0.0);
            for (i1, x1) in self.universe.into_iter().enumerate() {
                for (i2, x2) in rhs.universe.into_iter().enumerate() {
                    if f64::abs(*x - *x1 - *x2) < epsilon {
                        mu[i] += self.mu[i1] * rhs.mu[i2];
                    }
                }
            }
            if mu[i] > max {
                max = mu[i];
            }
        }
        for j in 0..self.mu.len() {
            mu[j] /= max;
        }

        MembershipRange::new(self.name.clone(), mu)
    }
}

impl Neg for MembershipRange {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut mu: Vec<f64> = Vec::new();
        for i in 0..self.mu.len() {
            mu.push(-self.mu[i]);
        }
        MembershipRange::new(self.name.clone(), mu)
    }
}

impl Sub for MembershipRange {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut mu: Vec<f64> = Vec::new();
        let mut max: f64 = -f64::INFINITY;
        let epsilon: f64 = 0.05 * (self.universe[1] - self.universe[0]);

        for (i, x) in self.universe.into_iter().enumerate() {
            mu.push(0.0);
            for (i1, x1) in self.universe.into_iter().enumerate() {
                for (i2, x2) in rhs.universe.into_iter().enumerate() {
                    if f64::abs(*x - *x1 + *x2) < epsilon {
                        mu[i] += self.mu[i1] * rhs.mu[i2];
                    }
                }
            }
            if mu[i] > max {
                max = mu[i];
            }
        }
        for j in 0..self.mu.len() {
            mu[j] /= max;
        }

        MembershipRange::new(self.name.clone(), mu)
    }
}

impl Mul for MembershipRange {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut mu: Vec<f64> = Vec::new();
        let mut max: f64 = -f64::INFINITY;
        let epsilon: f64 = 0.05 * (self.universe[1] - self.universe[0]);

        for (i, x) in self.universe.into_iter().enumerate() {
            mu.push(0.0);
            for (i1, x1) in self.universe.into_iter().enumerate() {
                for (i2, x2) in rhs.universe.into_iter().enumerate() {
                    if f64::abs(*x - *x1 * *x2) < epsilon {
                        mu[i] += self.mu[i1] * rhs.mu[i2];
                    }
                }
            }
            if mu[i] > max {
                max = mu[i];
            }
        }
        for j in 0..self.mu.len() {
            mu[j] /= max;
        }

        MembershipRange::new(self.name.clone(), mu)
    }
}

impl Div for MembershipRange {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut mu: Vec<f64> = Vec::new();
        let mut max: f64 = -f64::INFINITY;
        let epsilon: f64 = 0.05 * (self.universe[1] - self.universe[0]);

        for (i, x) in self.universe.into_iter().enumerate() {
            mu.push(0.0);
            for (i1, x1) in self.universe.into_iter().enumerate() {
                for (i2, x2) in rhs.universe.into_iter().enumerate() {
                    if f64::abs(*x - *x1 / *x2) < epsilon {
                        mu[i] += self.mu[i1] * rhs.mu[i2];
                    }
                }
            }
            if mu[i] > max {
                max = mu[i];
            }
        }
        for j in 0..self.mu.len() {
            mu[j] /= max;
        }

        MembershipRange::new(self.name.clone(), mu)
    }
}
*/
