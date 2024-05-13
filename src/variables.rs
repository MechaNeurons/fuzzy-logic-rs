use crate::membership_functions::{
    linear_membership, GetDegree, MembershipFunction, TSKMembershipFunction,
};
use crate::membership_ranges::MembershipRange;

#[derive(Debug, Clone)]
pub struct InputVariable {
    name: String,
    range: (f64, f64),
    mfs: Vec<MembershipFunction>,
}

impl InputVariable {
    pub fn new(name: String, range: (f64, f64)) -> Self {
        Self {
            name,
            range,
            mfs: Vec::new(),
        }
    }
    pub fn add_membership(&mut self, mf: MembershipFunction) {
        self.mfs.push(mf);
    }

    pub fn fuzzify(&self, idx: usize, x: f64) -> f64 {
        self.mfs[idx].get_degree(x)
    }
    pub fn membership_function_name(&self, idx: i32) -> String {
        self.mfs[idx as usize].get_name().clone()
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_range(&self) -> &(f64, f64) {
        &self.range
    }
}

#[derive(Debug)]
pub struct OutputVariable {
    name: String,
    mrs: Vec<MembershipRange>,
    universe: Vec<f64>,
}

impl OutputVariable {
    pub fn new(name: String, range: (f64, f64), n: i32) -> Self {
        let mut universe = Vec::new();
        let (start, stop) = range;
        let delta = (stop - start) / (n as f64);
        for i in 0..n {
            universe.push(start + delta * (i as f64))
        }
        Self {
            name,
            mrs: Vec::new(),
            universe,
        }
    }
    pub fn add_membership(&mut self, membership_rang: MembershipRange) {
        self.mrs.push(membership_rang)
    }

    pub fn get_mu(&self, idx: usize) -> &Vec<f64> {
        self.mrs[idx].get_mu()
    }

    pub fn get_universe(&self) -> &Vec<f64> {
        &self.universe
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_universe_by_idx(&self, idx: usize) -> f64 {
        self.universe[idx].clone()
    }
}

#[derive(Debug)]
pub struct TSKOutputVariable {
    name: String,
    mfs: Vec<TSKMembershipFunction>,
}

impl TSKOutputVariable {
    pub fn new(name: String) -> Self {
        Self {
            name,
            mfs: Vec::new(),
        }
    }

    pub fn add_membership(&mut self, membership: TSKMembershipFunction) {
        self.mfs.push(membership);
    }

    pub fn add_constant_membership(&mut self, value: f64) {
        self.mfs.push(TSKMembershipFunction::Constant(value));
    }

    pub fn add_linear_membership(&mut self, coefficients: Vec<f64>) {
        self.mfs.push(TSKMembershipFunction::Linear(coefficients));
    }

    pub fn get_mu(&self, idx: usize, input_vec: &Vec<f64>) -> f64 {
        match &self.mfs[idx] {
            TSKMembershipFunction::Constant(c) => *c,
            TSKMembershipFunction::Linear(coeff) => linear_membership(&coeff, input_vec),
            TSKMembershipFunction::Custom(fun) => fun(input_vec),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
