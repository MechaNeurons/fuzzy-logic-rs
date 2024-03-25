//pub mod memberships;

//#[allow(unused)]
//use memberships::{MemberShip, Universe};

pub mod membership_functions {

    pub trait GetDegree {
        fn get_degree(&self, x: f64) -> f64;
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
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
        Normal(Gaussian),
        Custom(Custom),
    }
    impl GetDegree for Kind {
        fn get_degree(&self, x: f64) -> f64 {
            match self {
                Self::Triangle(mf) => mf.get_degree(x),
                Self::Gaussian(mf) => mf.get_degree(x),
                Self::Custom(mf) => mf.get_degree(x),
                _ => 0.0,
            }
        }
    }

    pub type MFKind = Kind;

    #[derive(Debug, Clone)]
    pub struct MembershipFunction {
        pub name: String,
        pub kind: Kind,
    }

    pub type MF = MembershipFunction;

    impl GetDegree for MembershipFunction {
        fn get_degree(&self, x: f64) -> f64 {
            self.kind.get_degree(x)
        }
    }

    impl MembershipFunction {
        pub fn new(name: String, kind: Kind) -> Self {
            MF { name, kind }
        }
    }

    #[derive(Debug, Clone)]
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

    #[derive(Debug, Clone)]
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

    #[derive(Debug, Clone)]
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

    #[derive(Debug, Clone)]
    pub struct Custom {
        name: String,
        parameters: Vec<f64>,
        func: fn(f64, &Vec<f64>) -> f64,
    }

    impl Custom {
        pub fn new(name: String, parameters: Vec<f64>, func: fn(f64, &Vec<f64>) -> f64) -> Self {
            Self {
                name,
                parameters,
                func,
            }
        }
        pub fn get_name(&self) -> String {
            self.name.clone()
        }
    }

    impl GetDegree for Custom {
        fn get_degree(&self, x: f64) -> f64 {
            (self.func)(x, &self.parameters)
        }
    }
}

pub mod variables {

    use crate::membership_functions::{GetDegree, MembershipFunction};

    #[derive(Debug, Clone, PartialEq)]
    enum Kind {
        Input,
        Output,
    }
    #[derive(Debug, Clone)]
    pub struct Variables {
        name: String,
        kind: Kind,
        // range: (f64, f64),
        mfs: Vec<MembershipFunction>,
    }
    pub type Var = Variables;
    impl Variables {
        /*
        pub fn new(name: String, kind: Kind) -> Self {
            Variables {
                name,
                kind,
                mfs: Vec::new(),
            }
        }
        */

        pub fn new_input(name: String) -> Self {
            Var {
                name,
                kind: Kind::Input,
                mfs: Vec::new(),
            }
        }

        pub fn new_output(name: String) -> Self {
            Var {
                name,
                kind: Kind::Output,
                mfs: Vec::new(),
            }
        }

        pub fn add_membership(&mut self, mf: MembershipFunction) {
            self.mfs.push(mf);
        }

        pub fn fuzzify(&self, name: String, x: f64) -> f64 {
            assert_eq!(self.kind, Kind::Input);
            for mf in self.mfs.iter() {
                if mf.name == name {
                    return mf.get_degree(x);
                }
            }
            0.0
        }
    }
}

pub mod t_norms {

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
}

pub mod s_norms {

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
}

pub mod rule {

    #[derive(Debug)]
    enum Kind {
        OR,
        AND,
    }

    #[derive(Debug)]
    pub struct Rule {
        relations: Vec<i32>,
        weight: f64,
        method: Kind,
    }

    impl Rule {
        pub fn new_or(relations: Vec<i32>, weight: f64) -> Self {
            Self {
                relations,
                weight,
                method: Kind::OR,
            }
        }

        pub fn new_and(relations: Vec<i32>, weight: f64) -> Self {
            Self {
                relations,
                weight,
                method: Kind::AND,
            }
        }
    }
}

pub mod fuzzy_inference_system {
    use crate::rule::Rule;
    use crate::s_norms::SNorms;
    use crate::t_norms::TNorms;
    use crate::variables::Variables;

    #[derive(Debug)]
    pub struct MamdaniFuzzyInferenceSystem {
        s_norm: SNorms,
        t_norm: TNorms,
        rules: Vec<Rule>,
        inputs: Vec<Variables>,
        outputs: Vec<Variables>,
    }

    pub type MamdaniFIS = MamdaniFuzzyInferenceSystem;

    impl MamdaniFuzzyInferenceSystem {
        pub fn new(s_norm: SNorms, t_norm: TNorms) -> Self {
            Self {
                s_norm,
                t_norm,
                rules: Vec::new(),
                inputs: Vec::new(),
                outputs: Vec::new(),
            }
        }

        pub fn new_all(
            s_norm: SNorms,
            t_norm: TNorms,
            rules: Vec<Rule>,
            inputs: Vec<Variables>,
            outputs: Vec<Variables>,
        ) -> Self {
            Self {
                s_norm,
                t_norm,
                rules,
                inputs,
                outputs,
            }
        }

        pub fn add_input(&mut self, input: Variables) {
            self.inputs.push(input);
        }

        pub fn add_output(&mut self, output: Variables) {
            self.outputs.push(output);
        }

        pub fn add_rule(&mut self, rule: Rule) {
            self.rules.push(rule);
        }

        pub fn compute_s_norm(&self, fuzzified: Vec<f64>) -> f64 {
            self.s_norm.s_norm(fuzzified)
        }

        pub fn compute_t_norm(&self, fuzzified: Vec<f64>) -> f64 {
            self.t_norm.t_norm(fuzzified)
        }
        #[allow(unused)]
        pub fn compute_outputs(&self, input: f64) -> Vec<f64> {
            let mut out = Vec::new();

            let defuzzified: Vec<f64> = self
                .inputs
                .iter()
                .map(|e| e.fuzzify("L".to_string(), input))
                .collect();

            println!("{:#?}", defuzzified);

            // TODO add code for implications
            // TODO add code for inference system
            // TODO add code for aggregation
            // TODO add code for defuzzification

            out
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
