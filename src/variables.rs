use crate::membership_functions::{GetDegree, MembershipFunction};

#[derive(Debug, Clone, PartialEq)]
enum Kind {
    Input,
    Output,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Variables {
    name: String,
    kind: Kind,
    mfs: Vec<MembershipFunction>,
}

pub type Var = Variables;

impl Variables {
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

    pub fn fuzzify(&self, idx: usize, x: f64) -> f64 {
        assert_eq!(self.kind, Kind::Input);
        return self.mfs[idx].get_degree(x);
    }
    pub fn membership_function_name(&self, idx: i32) -> String {
        self.mfs[idx as usize].get_name().clone()
    }
}
