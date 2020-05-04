use super::Expression;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub expressions: Vec<Expression>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            name: String::from("Main"),
            expressions: Vec::new(),
        }
    }
}
