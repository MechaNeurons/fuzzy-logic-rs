use crate::aggregations::Aggregations;
use crate::defuzzifications::Defuzzifiers;
use crate::implications::Implications;
use crate::rules::{self, Rule};
use crate::s_norms::SNorms;
use crate::t_norms::TNorms;
use crate::variables::{InputVariables, OutputVariables};

#[derive(Debug)]
pub struct MamdaniFuzzyInferenceSystem {
    s_norm: SNorms,
    t_norm: TNorms,
    implication: Implications,
    aggregation: Aggregations,
    defuzzifier: Defuzzifiers,
    rules: Vec<Rule>,
    inputs: Vec<InputVariables>,
    outputs: Vec<OutputVariables>,
}


pub type MamdaniFIS = MamdaniFuzzyInferenceSystem;

impl MamdaniFuzzyInferenceSystem {
    pub fn new(
        s_norm: SNorms,
        t_norm: TNorms,
        implication: Implications,
        aggregation: Aggregations,
        defuzzifier: Defuzzifiers,
    ) -> Self {
        Self {
            s_norm,
            t_norm,
            implication,
            aggregation,
            defuzzifier,
            rules: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn new_all(
        s_norm: SNorms,
        t_norm: TNorms,
        implication: Implications,
        aggregation: Aggregations,
        defuzzifier: Defuzzifiers,
        rules: Vec<Rule>,
        inputs: Vec<InputVariables>,
        outputs: Vec<OutputVariables>,
    ) -> Self {
        Self {
            s_norm,
            t_norm,
            implication,
            aggregation,
            defuzzifier,
            rules,
            inputs,
            outputs,
        }
    }

    pub fn add_input(&mut self, input: InputVariables) {
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: OutputVariables) {
        if self.outputs.len() >= 1 {
            panic!("Warning: MIMO(Multiple Input Multiple Output) systems are not tested yet");
        }
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
    
    pub fn compute_outputs(&self, input_vec: Vec<f64>) -> Vec<f64> {
        let mut out = Vec::new();
        let size_inputs = self.inputs.len();
        let size_outputs = self.outputs.len();
        let size_rules = self.rules.len();

        let mut inference_output: Vec<f64> = Vec::new();
        for i in 0..size_rules {
            let mut fuzzified: Vec<f64> = Vec::new();
            for ii in 0..size_inputs {
                let idx = self.rules[i].relations()[ii] as usize;
                fuzzified.push(self.inputs[ii].fuzzify(idx, input_vec[ii]));
            }
            inference_output.push(match self.rules[i].kind() {
                rules::Kind::AND => self.compute_t_norm(fuzzified),
                rules::Kind::OR => self.compute_s_norm(fuzzified),
            });
        }

        let mut implication_output = Vec::new();
        for i in 0..size_rules {
            for ii in 0..size_outputs {
                let idx = self.rules[i].relations()[ii + size_inputs] as usize;
                let v1 = self.outputs[ii].get_mu(idx);
                implication_output.push(self.implication.implication(inference_output[i], v1));
            }
        }

        let aggregation_output = self.aggregation.aggregation(implication_output);
        out.push(
            self.defuzzifier
                .defuzzify(aggregation_output, self.outputs[0].get_universe()),
        );
        out
    }
}
