#[derive(Debug)]
pub enum Kind {
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
        assert!(weight <= 1.0, "Weight must be less or equal to 1.0");
        assert!(weight >= 0.0, "Weight must be less or equal to 1.0");
        Self {
            relations,
            weight,
            method: Kind::OR,
        }
    }

    pub fn new_and(relations: Vec<i32>, weight: f64) -> Self {
        assert!(weight <= 1.0, "Weight must be less or equal to 1.0");
        assert!(weight >= 0.0, "Weight must be less or equal to 1.0");
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

#[derive(Debug)]
pub enum OutputRelation {
    Constant,
    Linear,
    Custom,
}

#[derive(Debug)]
pub struct TSKRule {
    input_relations: Vec<i32>,
    output_relations: Vec<OutputRelation>,
    weight: f64,
    method: Kind,
}

impl TSKRule {
    pub fn new_or(
        input_relations: Vec<i32>,
        output_relations: Vec<OutputRelation>,
        weight: f64,
    ) -> Self {
        assert!(weight <= 1.0, "Weight must be less or equal to 1.0");
        assert!(weight >= 0.0, "Weight must be less or equal to 1.0");
        Self {
            input_relations,
            output_relations,
            weight,
            method: Kind::OR,
        }
    }

    pub fn new_and(
        input_relations: Vec<i32>,
        output_relations: Vec<OutputRelation>,
        weight: f64,
    ) -> Self {
        assert!(weight <= 1.0, "Weight must be less or equal to 1.0");
        assert!(weight >= 0.0, "Weight must be less or equal to 1.0");
        Self {
            input_relations,
            output_relations,
            weight,
            method: Kind::AND,
        }
    }

    pub fn get_kind(&self) -> &Kind {
        &self.method
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    pub fn get_input_rules(&self) -> &[i32] {
        &self.input_relations
    }

    pub fn get_output_rules(&self) -> &[OutputRelation] {
        &self.output_relations
    }
}
