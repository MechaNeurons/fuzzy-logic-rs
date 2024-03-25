use crate::rules::{self, Rule};
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
    pub fn compute_outputs(&self, input_vec: Vec<f64>) -> Vec<f64> {
        let mut out = Vec::new();

        let mut fuzzified: Vec<f64> = Vec::new();
        for i in 0..self.rules.len() {
            let mut temp: Vec<f64> = Vec::new();
            for ii in 0..self.inputs.len() {
                let idx = self.rules[i].relations()[ii];
                temp.push(self.inputs[ii].fuzzify(
                    &self.inputs[ii].membership_function_name(idx),
                    input_vec[ii],
                ));
            }
            let k = match self.rules[i].kind() {
                rules::Kind::AND => self.t_norm.t_norm(temp),
                rules::Kind::OR => self.s_norm.s_norm(temp),
            };
            fuzzified.push(k);
        }

        out
    }
}
