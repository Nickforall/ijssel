use super::tokenizer::{BinaryOperator, Keyword::*, Token, TokenValue, TokenValue::*};

use std::iter::Peekable;
use std::slice::Iter;

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

#[derive(Debug)]
pub enum Expression {
    Function(Box<FunctionExpression>),
    Binary(Box<BinaryExpression>),
    NumberLiteral(Box<NumberLiteralExpression>),
    // Block(Box<BlockExpression>),
}

#[derive(Debug)]
pub struct NumberLiteralExpression {
    pub number: f64,
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

impl NumberLiteralExpression {
    pub fn new(number: f64) -> Self {
        NumberLiteralExpression { number }
    }
}

#[derive(Debug)]
pub struct FunctionExpression {
    pub name: String,
    pub body: BlockExpression,
}

impl FunctionExpression {
    pub fn new(name: &str, block: BlockExpression) -> Self {
        Self {
            name: String::from(name),
            body: block,
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
pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    pub module: Module,
}

impl Parser<'_> {
    pub fn new(tokens: &'_ Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.iter().peekable(),
            module: Module::new(),
        }
    }

    pub fn parse_expression(&mut self) -> Expression {
        let atom = self.parse_atom();
        return self.parse_maybe_binary(atom, 0);
    }

    pub fn parse_atom(&mut self) -> Expression {
        let peek = self.tokens.peek().expect("Unexpected EOF");
        match &peek.value {
            TokenValue::NumConst(float) => {
                self.tokens.by_ref().next();
                Expression::NumberLiteral(Box::new(NumberLiteralExpression::new(*float)))
            }
            val => panic!(
                "Expected an expression, got token {:?} which cannot compose an expression",
                val
            ),
        }
    }

    pub fn parse_maybe_binary(&mut self, left: Expression, precedence: u16) -> Expression {
        let peek = self.tokens.peek().expect("Unexpected EOF");

        if let Operator(op) = &peek.value {
            let right_precedence = op.precedence();

            if right_precedence > precedence {
                let _token = self.tokens.by_ref().next();

                let expression = self.parse_atom();
                let binary_expression = BinaryExpression::new(
                    *op,
                    left,
                    self.parse_maybe_binary(expression, right_precedence),
                );

                return self.parse_maybe_binary(
                    Expression::Binary(Box::new(binary_expression)),
                    precedence,
                );
            }
        }

        return left;
    }

    pub fn parse_block(&mut self) -> BlockExpression {
        let open_token = self.tokens.by_ref().peek().expect("Unexpected EOF");
        if let Keyword(Do) = &open_token.value {
            self.tokens.by_ref().next();
        }

        let mut expressions = Vec::new();

        while self.tokens.peek().is_some() {
            let possible_end_token = self.tokens.by_ref().peek().expect("Unexpected EOF");
            if let Keyword(End) = &possible_end_token.value {
                self.tokens.by_ref().next();
                break;
            }

            let expression = self.parse_expression();
            expressions.push(expression);
        }

        return BlockExpression::new(expressions);
    }

    pub fn parse_function(&mut self) -> FunctionExpression {
        let fn_keyword = self.tokens.by_ref().peek().expect("Unexpected EOF");

        if let Keyword(Fn) = &fn_keyword.value {
            self.tokens.by_ref().next();

            let name_token = self.tokens.by_ref().peek().expect("Unexpected EOF");
            if let Identifier(name) = &name_token.value {
                self.tokens.by_ref().next();
                FunctionExpression::new(name, self.parse_block())
            } else {
                panic!("Unexpected token, expected Identifier ")
            }
        } else {
            panic!("Unexpected token, expected Keyword `fn` ")
        }
    }

    pub fn parse_top_level_expression(&mut self) {
        let peek = self
            .tokens
            .by_ref()
            .peek()
            .expect("Expected token, got None");

        let expression: Option<Expression> = match &peek.value {
            Keyword(Fn) => Some(Expression::Function(Box::new(self.parse_function()))),
            token => panic!("Unexpected token, {:?}", token),
        };

        if let Some(expression) = expression {
            self.module.expressions.push(expression)
        }
    }

    pub fn parse_module(&mut self) {
        while self.tokens.peek().is_some() {
            self.parse_top_level_expression()
        }
    }
}