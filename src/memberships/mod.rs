#[derive(Debug)]
pub struct Universe<const N: usize> {
    pub data: [f64; N],
}

#[allow(unused)]
impl<const N: usize> Universe<N> {
    pub fn new(start: f64, end: f64) -> Self {
        let mut data = [0.0; N];
        let n = N as f64;
        let dt: f64 = (end - start) / n;
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

/*
    Triangular membership function
    please note that a < b < c.
*/

#[derive(Debug)]
pub struct Triangle<const N: usize> {
    // universe: &'a Universe<N>,
    mu: [f64; N],
}

impl<const N: usize> Triangle<N> {
    pub fn new(universe: &Universe<N>, a: f64, b: f64, c: f64) -> Self {
        assert!(a < b);
        assert!(b < c);
        let mut mu: [f64; N] = [0.0; N];
        for (i, value) in universe.into_iter().enumerate() {
            let x = value.clone();
            let data;
            if x <= a {
                data = 0.0;
            } else if x <= b {
                data = (x - a) / (b - a);
            } else if x <= c {
                data = (c - x) / (c - b);
            } else {
                data = 0.0;
            }
            mu[i] = data;
        }
        Self { mu }
    }
}
impl<const N: usize> IntoIterator for Triangle<N> {
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.mu.into_iter()
    }
}
