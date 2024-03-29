#[derive(Debug)]
pub enum Kind {
    OR,
    AND,
}
#[allow(unused)]
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
    pub fn get_rules(&self) -> &[i32] {
        &self.relations[..]
    }

    pub fn get_kind(&self) -> &Kind {
        &self.method
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    pub fn get_input_rules(&self, input_size: usize) -> &[i32] {
        &self.relations[..input_size]
    }

    pub fn get_output_rules(&self, input_size: usize) -> &[i32] {
        &self.relations[input_size..]
    }
}
