// pub mod membership_functions;

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
            Self::Trapezoid(mf) => mf.get_degree(x),
            Self::Gaussian(mf) => mf.get_degree(x),
            Self::Custom(mf) => mf.get_degree(x),
            _ => 0.0,
        }
    }
}

pub type MFKind = Kind;

#[derive(Debug, Clone)]
pub struct MembershipFunction {
    name: String,
    kind: Kind,
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
    pub fn get_name(&self) -> &String {
        &self.name
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
        assert!(a <= b, "a must be less than or equal to b");
        assert!(b <= c, "b must be less than or equal to c");
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
        assert!(a <= b, "a must be less than b");
        assert!(b <= c, "b must be less than c");
        assert!(c <= d, "c must be less than d");
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
