use crate::membership_functions::{GetDegree, MembershipFunction};
use crate::membership_ranges::MembershipRange;
/*
#[derive(Debug, Clone, PartialEq)]
#[derive(Debug)]
enum Memberships {
    Function(MembershipFunction),
    Range(MembershipRange),
}
*/
#[derive(Debug, Clone)]
pub struct InputVariables {
    name: String,
    range: (f64, f64),
    mfs: Vec<MembershipFunction>,
}

impl InputVariables {
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
pub struct OutputVariables {
    name: String,
    mrs: Vec<MembershipRange>,
    universe: Vec<f64>,
}

impl OutputVariables {
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
