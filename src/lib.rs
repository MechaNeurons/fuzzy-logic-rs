//pub mod memberships;

//#[allow(unused)]
//use memberships::{MemberShip, Universe};

pub mod memberships {

    pub trait GetDegree {
        fn get_degree(&self, x: f64) -> f64;
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Kind {
        Triangle(Triangle),
        Trapezoid(Trapezoid),
        LinearZ,
        LinearS,
        StepDown,
        StepUp,
        Gaussian(Gaussian),
        DoubleGaussian,
        Bell,
        Normal,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct MembershipFunction {
        pub name: String,
        pub kind: Kind,
    }

    #[derive(Debug)]
    pub struct Triangle {
        a: f64,
        b: f64,
        c: f64,
    }

    impl Triangle {
        pub fn new(a: f64, b: f64, c: f64) -> Self {
            assert!(a < b, "a must be less than or equal to b");
            assert!(b < c, "b must be less than or equal to c");
            Self { a, b, c }
        }
    }

    impl GetDegree for Triangle {
        fn get_degree(&self, x: f64) -> f64 {
            if x < self.a {
                0.0
            } else if x < self.b {
                (x - self.a) / (self.b - self.a)
            } else if x < self.c {
                (self.c - x) / (self.c - self.b)
            } else {
                0.0
            }
        }
    }

    #[derive(Debug)]
    pub struct Gaussian {
        mean: f64,
        variance: f64,
    }
    impl Gaussian {
        pub fn new(mean: f64, variance: f64) -> Self {
            Self { mean, variance }
        }
    }

    impl GetDegree for Gaussian {
        fn get_degree(&self, x: f64) -> f64 {
            f64::exp(-0.5 * f64::powi((x - self.mean) / self.variance, 2))
        }
    }

    #[derive(Debug)]
    pub struct Trapezoid {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    }

    impl Trapezoid {
        pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
            assert!(a < b, "a must be less than b");
            assert!(b < c, "b must be less than c");
            assert!(c < d, "c must be less than d");
            Self { a, b, c, d }
        }
    }

    impl GetDegree for Trapezoid {
        fn get_degree(&self, x: f64) -> f64 {
            if x <= self.a {
                0.0
            } else if x <= self.b {
                (x - self.a) / (self.b - self.a)
            } else if x <= self.c {
                1.0
            } else if x <= self.d {
                (self.d - x) / (self.d - self.c)
            } else {
                0.0
            }
        }
    }
}

pub mod fuzzy_inference_system {
    use crate::memberships::*;

    pub struct MamdaniFIS {
        inputs: Vec<Variables>,
        outputs: Vec<Variables>,
    }
    pub struct Variables {
        name: String,
        range: (f64, f64),
        mfs: Vec<MembershipFunction>,
    }
    impl Variables {
        pub fn new(name: String, range: (f64, f64)) -> Self {
            Variables {
                name,
                range,
                mfs: Vec::new(),
            }
        }
    }
}

/*#[cfg(test)]
#[allow(dead_code, unused)]:
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let u1: Universe<100> = Universe::new(0.0, 11.0);
        let mf1 = MemberShip::new_triangle(&u1, 1.0, 2.0, 3.0);
        let mf2 = MemberShip::new_triangle(&u1, 2.5, 5.0, 7.5);
        println!("{:?}", mf2);
        println!("{:?}", -mf2);
    }
}
*/
