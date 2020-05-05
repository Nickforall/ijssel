use super::Expression;

#[derive(Debug)]
pub struct AstModule {
    pub name: String,
    pub expressions: Vec<Expression>,
}

impl AstModule {
    pub fn new() -> Self {
        Self {
            name: String::from("Main"),
            expressions: Vec::new(),
        }
    }
}
