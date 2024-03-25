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
    pub fn relations(&self) -> &Vec<i32> {
        &self.relations
    }

    pub fn kind(&self) -> &Kind {
        &self.method
    }
}
