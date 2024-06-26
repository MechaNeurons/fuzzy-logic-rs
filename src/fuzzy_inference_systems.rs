use crate::aggregations::Aggregations;
use crate::defuzzifications::{Defuzzifiers, TSKDefuzzifiers};
use crate::implications::Implications;
use crate::rules::{self, Rule};
use crate::s_norms::SNorms;
use crate::t_norms::TNorms;
use crate::variables::{InputVariable, OutputVariable, TSKOutputVariable};

#[derive(Debug)]
pub struct MamdaniFuzzyInferenceSystem {
    s_norm: SNorms,
    t_norm: TNorms,
    implication: Implications,
    aggregation: Aggregations,
    defuzzifier: Defuzzifiers,
    rules: Vec<Rule>,
    inputs: Vec<InputVariable>,
    outputs: Vec<OutputVariable>,
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
        inputs: Vec<InputVariable>,
        outputs: Vec<OutputVariable>,
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

    pub fn add_input(&mut self, input: InputVariable) {
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: OutputVariable) {
        self.outputs.push(output);
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn get_s_norm(&self, fuzzified: &[f64]) -> f64 {
        self.s_norm.s_norm(fuzzified)
    }

    pub fn get_t_norm(&self, fuzzified: &[f64]) -> f64 {
        self.t_norm.t_norm(fuzzified)
    }

    pub fn get_rules(&self, rule_index: usize) -> &[i32] {
        self.rules[rule_index].get_rules()
    }

    pub fn get_input_rules(&self, rule_index: usize) -> &[i32] {
        self.rules[rule_index].get_input_rules(self.inputs.len())
    }

    pub fn get_output_rules(&self, rule_index: usize) -> &[i32] {
        self.rules[rule_index].get_output_rules(self.inputs.len())
    }

    pub fn fuzzification(&self, input_vec: Vec<f64>) -> Vec<Vec<f64>> {
        let mut fuzzified: Vec<Vec<f64>> = Vec::new();
        for i in 0..self.rules.len() {
            let input_rule: &[i32] = self.get_input_rules(i);
            let mut temp_vec: Vec<f64> = Vec::new();
            for ii in 0..self.inputs.len() {
                let index;
                let complement;
                if input_rule[ii] < 0 {
                    index = (-input_rule[ii]) as usize;
                    complement = true;
                } else {
                    index = input_rule[ii] as usize;
                    complement = false;
                }
                let fuzzed: f64 = self.inputs[ii].fuzzify(index, input_vec[ii]);
                temp_vec.push(match complement {
                    true => 1.0 - fuzzed,
                    false => fuzzed,
                });
                //temp_vec.push(self.inputs[ii].fuzzify(input_rule[ii] as usize, input_vec[ii]));
            }
            fuzzified.push(temp_vec);
        }
        fuzzified
    }

    pub fn connect_inputs(&self, fuzzified: Vec<Vec<f64>>) -> Vec<f64> {
        fuzzified
            .into_iter()
            .zip(&self.rules)
            .map(|(fuzz, rule)| match rule.get_kind() {
                rules::Kind::OR => self.get_s_norm(&fuzz),
                rules::Kind::AND => self.get_t_norm(&fuzz),
            })
            .collect()
    }

    pub fn weighed_inputs(&self, connected_inputs: Vec<f64>) -> Vec<f64> {
        connected_inputs
            .into_iter()
            .zip(&self.rules)
            .map(|(mu, rule)| rule.get_weight() * mu)
            .collect()
    }

    pub fn implication(&self, connected_inputs: Vec<f64>) -> Vec<Vec<Vec<f64>>> {
        let mut implication_vec = Vec::new();
        for i in 0..self.outputs.len() {
            let mut temp_vec = Vec::new();
            for ii in 0..self.rules.len() {
                let output_rule = self.get_output_rules(ii);
                let index;
                let complement;
                if output_rule[i] < 0 {
                    index = (-output_rule[i]) as usize;
                    complement = true;
                } else {
                    index = output_rule[i] as usize;
                    complement = false;
                }
                let range: Vec<f64> = self.outputs[i]
                    .get_mu(index)
                    .into_iter()
                    .map(|e| if complement { 1.0 - e } else { e.to_owned() })
                    .collect();

                temp_vec.push(self.implication.implication(connected_inputs[ii], &range));
            }
            implication_vec.push(temp_vec);
        }
        implication_vec
    }

    pub fn aggregation(&self, implication_vec: Vec<Vec<Vec<f64>>>) -> Vec<Vec<f64>> {
        implication_vec
            .into_iter()
            .map(|vec| self.aggregation.aggregation(&vec))
            .collect()
    }

    pub fn defuzzification(&self, aggregation_vec: Vec<Vec<f64>>) -> Vec<f64> {
        aggregation_vec
            .into_iter()
            .enumerate()
            .map(|(index, aggregated)| {
                self.defuzzifier
                    .defuzzify(aggregated, self.outputs[index].get_universe())
            })
            .collect()
    }

    pub fn compute_outputs(&self, input_vec: Vec<f64>) -> Vec<f64> {
        assert!(self.inputs.len() != 0, "You must add at least one INPUT");
        assert!(self.outputs.len() != 0, "You must add at least one OUTPUT");
        assert!(self.rules.len() != 0, "You must add at least one RULE");

        // 1. fuzzification
        let fuzzified = self.fuzzification(input_vec);
        let connected_inputs = self.connect_inputs(fuzzified);
        let weighted_inputs = self.weighed_inputs(connected_inputs);
        // 2. implication
        let implication_vec = self.implication(weighted_inputs);
        // 3. aggregation
        let aggregation_vec = self.aggregation(implication_vec);
        // 4. defuzzification
        self.defuzzification(aggregation_vec)
    }
}

#[derive(Debug)]
pub struct TSKFuzzyInferenceSystem {
    s_norm: SNorms,
    t_norm: TNorms,
    defuzzification: TSKDefuzzifiers,
    rules: Vec<Rule>,
    inputs: Vec<InputVariable>,
    outputs: Vec<TSKOutputVariable>,
}

pub type TSKFIS = TSKFuzzyInferenceSystem;

impl TSKFuzzyInferenceSystem {
    pub fn new(s_norm: SNorms, t_norm: TNorms, defuzzification: TSKDefuzzifiers) -> Self {
        Self {
            s_norm,
            t_norm,
            defuzzification,
            rules: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_input(&mut self, input: InputVariable) {
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, variable: TSKOutputVariable) {
        self.outputs.push(variable);
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn get_s_norm(&self, fuzzified: &[f64]) -> f64 {
        self.s_norm.s_norm(fuzzified)
    }

    pub fn get_t_norm(&self, fuzzified: &[f64]) -> f64 {
        self.t_norm.t_norm(fuzzified)
    }

    pub fn get_rules(&self, rule_index: usize) -> &[i32] {
        self.rules[rule_index].get_rules()
    }

    pub fn get_input_rules(&self, rule_index: usize) -> &[i32] {
        self.rules[rule_index].get_input_rules(self.inputs.len())
    }

    pub fn get_output_rules(&self, rule_index: usize) -> &[i32] {
        self.rules[rule_index].get_output_rules(self.inputs.len())
    }

    pub fn fuzzification(&self, input_vec: Vec<f64>) -> Vec<Vec<f64>> {
        let mut fuzzified: Vec<Vec<f64>> = Vec::new();
        for i in 0..self.rules.len() {
            let input_rule = self.get_input_rules(i);
            let mut temp_vec: Vec<f64> = Vec::new();
            for ii in 0..self.inputs.len() {
                let index;
                let complement;
                if input_rule[ii] < 0 {
                    index = (-input_rule[ii]) as usize;
                    complement = true;
                } else {
                    index = input_rule[ii] as usize;
                    complement = false;
                }
                let fuzzed = self.inputs[ii].fuzzify(index, input_vec[ii]);
                temp_vec.push(match complement {
                    true => 1.0 - fuzzed,
                    false => fuzzed,
                });
                //temp_vec.push(self.inputs[ii].fuzzify(input_rule[ii] as usize, input_vec[ii]));
            }
            fuzzified.push(temp_vec);
        }
        fuzzified
    }

    pub fn connect_inputs(&self, fuzzified: Vec<Vec<f64>>) -> Vec<f64> {
        fuzzified
            .into_iter()
            .zip(&self.rules)
            .map(|(fuzz, rule)| match rule.get_kind() {
                rules::Kind::OR => self.get_s_norm(&fuzz),
                rules::Kind::AND => self.get_t_norm(&fuzz),
            })
            .collect()
    }

    pub fn weighed_inputs(&self, connected_inputs: Vec<f64>) -> Vec<f64> {
        connected_inputs
            .into_iter()
            .zip(&self.rules)
            .map(|(mu, rule)| rule.get_weight() * mu)
            .collect()
    }

    pub fn get_mu(&self, input_vec: &Vec<f64>) -> Vec<Vec<f64>> {
        let mut output = Vec::new();
        for i in 0..self.outputs.len() {
            let mut temp_vec = Vec::new();
            for ii in 0..self.rules.len() {
                let output_rule = self.rules[ii].get_output_rules(self.inputs.len());
                temp_vec.push(
                    self.outputs[i].get_mu(output_rule[i as usize].try_into().unwrap(), &input_vec),
                )
            }
            output.push(temp_vec);
        }
        output
    }

    pub fn compute_outputs(&self, input: Vec<f64>) -> Vec<f64> {
        let mut output = Vec::new();

        // 1 - fuzzification
        let fuzzified = self.fuzzification(input.clone());
        let connected_inputs = self.connect_inputs(fuzzified);
        let weighted_input = self.weighed_inputs(connected_inputs);
        // weighted_inputs

        // 2 - implication
        let mu_vec = self.get_mu(&input);
        for i in 0..self.outputs.len() {
            output.push(self.defuzzification.defuzzify(&mu_vec[i], &weighted_input));
        }

        output
    }
}
