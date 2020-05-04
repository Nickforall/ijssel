use super::tokenizer::BinaryOperator;

#[derive(Debug)]
pub struct NumberLiteralExpression {
    pub number: f64,
}

impl NumberLiteralExpression {
    pub fn new(number: f64) -> Self {
        NumberLiteralExpression { number }
    }
}

#[derive(Debug)]
pub struct VariableExpression {
    pub binding: String,
}

impl VariableExpression {
    pub fn new(binding: String) -> Self {
        VariableExpression { binding }
    }
}

#[derive(Debug)]
pub struct BlockExpression {
    pub expressions: Vec<Expression>,
}

impl BlockExpression {
    pub fn new(expressions: Vec<Expression>) -> Self {
        BlockExpression { expressions }
    }
}

#[derive(Debug)]
pub struct FunctionExpression {
    pub name: String,
    pub body: BlockExpression,
    pub arguments: Vec<FunctionArgument>,
}

#[derive(Debug)]
pub struct ExternFunctionExpression {
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
}

#[derive(Clone, Debug)]
pub struct FunctionArgument {
    pub binding_name: String,
}

impl FunctionArgument {
    pub fn new(binding_name: String) -> Self {
        Self { binding_name }
    }
}

impl FunctionExpression {
    pub fn new(name: &str, body: BlockExpression, args: Vec<FunctionArgument>) -> Self {
        Self {
            name: String::from(name),
            arguments: args,
            body,
        }
    }
}

impl ExternFunctionExpression {
    pub fn new(name: &str, args: Vec<FunctionArgument>) -> Self {
        Self {
            name: String::from(name),
            arguments: args,
        }
    }
}

#[derive(Debug)]
pub struct CallExpression {
    pub function_name: String,
    pub args: Vec<Expression>,
}

impl CallExpression {
    pub fn new(function_name: String, args: Vec<Expression>) -> Self {
        Self {
            function_name,
            args,
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Expression,
    pub right: Expression,
    pub operator: BinaryOperator,
}

impl BinaryExpression {
    pub fn new(operator: BinaryOperator, left: Expression, right: Expression) -> Self {
        BinaryExpression {
            left,
            right,
            operator,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Function(Box<FunctionExpression>),
    ExternFunction(Box<ExternFunctionExpression>),
    Binary(Box<BinaryExpression>),
    NumberLiteral(Box<NumberLiteralExpression>),
    Variable(Box<VariableExpression>),
    Call(Box<CallExpression>),
    // Block(Box<BlockExpression>),
}
