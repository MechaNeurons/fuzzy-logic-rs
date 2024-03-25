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
    // range: (f64, f64),
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

    pub fn fuzzify(&self, name: &String, x: f64) -> f64 {
        assert_eq!(self.kind, Kind::Input);
        for mf in self.mfs.iter() {
            if mf.get_name() == name {
                return mf.get_degree(x);
            }
        }
        0.0
    }
    pub fn membership_function_name(&self, idx: i32) -> String {
        let mf = &self.mfs[idx as usize];
        mf.get_name().clone()
    }
}
