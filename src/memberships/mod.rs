use std::{
    ops::{Add, Div, Index, Mul, Neg, Sub},
    usize,
};
// #[derive(Debug)]
// enum MembershipsKind{
//     Custom,
//     Triangle,
//     Trapezoid,
//     Gaussian,
// }

#[derive(Debug)]
pub struct Universe<const N: usize> {
    pub data: [f64; N],
}

impl<const N: usize> Index<usize> for Universe<N> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[allow(unused)]
impl<const N: usize> Universe<N> {
    pub fn new(start: f64, end: f64) -> Self {
        let mut data = [0.0; N];
        let dt: f64 = (end - start) / (N as f64);
        for i in 0..N {
            data[i] = start + (i as f64) * dt;
        }
        Self { data }
    }
}

impl<const N: usize> IntoIterator for Universe<N> {
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a Universe<N> {
    type Item = &'a f64;
    type IntoIter = std::slice::Iter<'a, f64>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

#[derive(Debug)]
pub struct MemberShip<'a, const N: usize> {
    universe: &'a Universe<N>,
    mu: [f64; N],
    // kind: MembershipsKind,
}

impl<'a, const N: usize> Add for MemberShip<'a, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.universe as *const _, rhs.universe as *const _,
            "Universes must be the same"
        );
        let mut mu: [f64; N] = [0.0; N];
        let mut max = -f64::INFINITY;
        let epsilon = 0.05 * (self.universe[1] - self.universe[0]);

        for (i, x) in self.universe.into_iter().enumerate() {
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
        for j in 0..N {
            mu[j] /= max;
        }

        MemberShip::new(self.universe, mu)
    }
}

impl<'a, const N: usize> Neg for MemberShip<'a, N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut mu = [0.0; N];
        for i in 0..N {
            mu[i] = -self.mu[i];
        }
        MemberShip::new(self.universe, mu)
    }
}

impl<'a, const N: usize> Sub for MemberShip<'a, N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.universe as *const _, rhs.universe as *const _,
            "Universes must be the same"
        );
        let mut mu: [f64; N] = [0.0; N];
        let mut max = -f64::INFINITY;
        let epsilon = 0.05 * (self.universe[1] - self.universe[0]);

        for (i, x) in self.universe.into_iter().enumerate() {
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
        for j in 0..N {
            mu[j] /= max;
        }

        MemberShip::new(self.universe, mu)
    }
}

impl<'a, const N: usize> Mul for MemberShip<'a, N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.universe as *const _, rhs.universe as *const _,
            "Universes must be the same"
        );
        let mut mu: [f64; N] = [0.0; N];
        let mut max = -f64::INFINITY;
        let epsilon = 0.05 * (self.universe[1] - self.universe[0]);

        for (i, x) in self.universe.into_iter().enumerate() {
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
        for j in 0..N {
            mu[j] /= max;
        }

        MemberShip::new(self.universe, mu)
    }
}

impl<'a, const N: usize> Div for MemberShip<'a, N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.universe as *const _, rhs.universe as *const _,
            "Universes must be the same"
        );
        let mut mu: [f64; N] = [0.0; N];
        let mut max = -f64::INFINITY;
        let epsilon = 0.05 * (self.universe[1] - self.universe[0]);

        for (i, x) in self.universe.into_iter().enumerate() {
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
        for j in 0..N {
            mu[j] /= max;
        }

        MemberShip::new(self.universe, mu)
    }
}

impl<'a, const N: usize> MemberShip<'a, N> {
    pub fn new(universe: &'a Universe<N>, mu: [f64; N]) -> Self {
        Self { universe, mu }
    }
    pub fn new_triangle(universe: &'a Universe<N>, a: f64, b: f64, c: f64) -> Self {
        assert!(a < b, "a must be less than b");
        assert!(b < c, "b must be less that c");
        let mut mu: [f64; N] = [0.0; N];
        for (i, x) in universe.into_iter().enumerate() {
            let data;
            if *x <= a {
                data = 0.0;
            } else if *x <= b {
                data = (*x - a) / (b - a);
            } else if *x <= c {
                data = (c - *x) / (c - b);
            } else {
                data = 0.0;
            }
            mu[i] = data;
        }
        Self { universe, mu }
    }
    pub fn new_trapezoid(universe: &'a Universe<N>, a: f64, b: f64, c: f64, d: f64) -> Self {
        // assert!(universe[0]<a);
        assert!(a < b, "a must be less than b");
        assert!(b < c, "b must be less than c");
        assert!(c < d, "c must be less than d");
        let mut mu: [f64; N] = [0.0; N];
        let mut data: f64;
        for (i, value) in universe.into_iter().enumerate() {
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
            mu[i] = data;
        }
        Self { universe, mu }
    }

    pub fn new_linearz(universe: &'a Universe<N>, a: f64, b: f64) -> Self {
        assert!(a < b);
        let mut mu = [0.0; N];
        let mut data: f64;
        for (i, x) in universe.into_iter().enumerate() {
            if *x < a {
                data = 1.0;
            } else if *x < b {
                data = (a - *x) / (a - b);
            } else {
                data = 0.0;
            }
            mu[i] = data;
        }
        Self { universe, mu }
    }

    pub fn new_linears(universe: &'a Universe<N>, a: f64, b: f64) -> Self {
        assert!(a < b);
        let mut mu = [0.0; N];
        let mut data: f64;
        for (i, x) in universe.into_iter().enumerate() {
            if *x < a {
                data = 0.0;
            } else if *x < b {
                data = (*x - a) / (b - a);
            } else {
                data = 1.0;
            }
            mu[i] = data;
        }
        Self { universe, mu }
    }

    pub fn new_step_down(universe: &'a Universe<N>, a: f64) -> Self {
        let mut mu = [0.0; N];
        for (i, x) in universe.into_iter().enumerate() {
            let mut data: f64 = 1.0;
            if *x > a {
                data = 0.0;
            }
            mu[i] = data;
        }
        Self { universe, mu }
    }

    pub fn new_step_up(universe: &'a Universe<N>, a: f64) -> Self {
        let mut mu = [0.0; N];
        for (i, x) in universe.into_iter().enumerate() {
            let mut data: f64 = 0.0;
            if *x > a {
                data = 1.0;
            }
            mu[i] = data;
        }
        Self { universe, mu }
    }

    pub fn new_gaussian(universe: &'a Universe<N>, mean: f64, variance: f64) -> Self {
        let mut mu: [f64; N] = [0.0; N];
        for (i, x) in universe.into_iter().enumerate() {
            let data: f64 = f64::exp(-0.5 * f64::powi((*x - mean) / variance, 2));
            mu[i] = data;
        }
        Self { universe, mu }
    }

    pub fn new_double_gaussian(
        universe: &'a Universe<N>,
        mean1: f64,
        variance1: f64,
        mean2: f64,
        variance2: f64,
    ) -> Self {
        assert!(mean1 <= mean2, "mean1 must be less than mean2");
        let mut mu: [f64; N] = [0.0; N];
        for (i, x) in universe.into_iter().enumerate() {
            let data: f64;
            if *x < mean1 {
                data = f64::exp(-0.5 * f64::powi((*x - mean1) / variance1, 2));
            } else if *x < mean2 {
                data = 1.0;
            } else {
                data = f64::exp(-0.5 * f64::powi((*x - mean2) / variance2, 2));
            }
            mu[i] = data;
        }
        Self { universe, mu }
    }

    pub fn new_bell(universe: &'a Universe<N>, width: f64, shape: f64, center: f64) -> Self {
        let mut mu: [f64; N] = [0.0; N];
        for (i, x) in universe.into_iter().enumerate() {
            let data = 1.0 / (1.0 + f64::powf(f64::abs((*x - center) / width), 2.0 * shape));
            mu[i] = data;
        }
        Self { universe, mu }
    }
}

impl<'a, const N: usize> IntoIterator for MemberShip<'a, N> {
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.mu.into_iter()
    }
}
