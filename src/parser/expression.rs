use super::tokenizer::BinaryOperator;
use super::Type;

#[derive(Clone, Debug)]
pub struct NumberLiteralExpression {
    pub number: f64,
    pub return_type: Type,
}

impl NumberLiteralExpression {
    pub fn new(number: f64) -> Self {
        NumberLiteralExpression {
            number,
            return_type: Type::Int32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VariableExpression {
    pub binding: String,
    pub return_type: Type,
}

impl VariableExpression {
    pub fn new(binding: String) -> Self {
        VariableExpression {
            binding,
            return_type: Type::Unknown,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BlockExpression {
    pub expressions: Vec<Expression>,
    pub return_type: Type,
}

impl BlockExpression {
    pub fn new(expressions: Vec<Expression>) -> Self {
        BlockExpression {
            expressions,
            return_type: Type::Unknown,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionExpression {
    pub name: String,
    pub body: BlockExpression,
    pub arguments: Vec<FunctionArgument>,
}

#[derive(Clone, Debug)]
pub struct ExternFunctionExpression {
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Type,
}

#[derive(Clone, Debug)]
pub struct FunctionArgument {
    pub binding_name: String,
    pub input_type: Type,
}

impl FunctionArgument {
    pub fn new(binding_name: String, input_type: Type) -> Self {
        Self {
            binding_name,
            input_type,
        }
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
            return_type: Type::Int32,
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Expression {
    Function(Box<FunctionExpression>),
    ExternFunction(Box<ExternFunctionExpression>),
    Binary(Box<BinaryExpression>),
    NumberLiteral(Box<NumberLiteralExpression>),
    Variable(Box<VariableExpression>),
    Call(Box<CallExpression>),
    // Block(Box<BlockExpression>),
}
