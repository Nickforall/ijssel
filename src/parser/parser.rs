use super::expression::*;
use super::tokenizer::{Keyword::*, Token, TokenValue, TokenValue::*};
use super::types::Type;
use super::AstModule;

use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    pub module: AstModule,
}

impl Parser<'_> {
    pub fn new(tokens: &'_ Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.iter().peekable(),
            module: AstModule::new(),
        }
    }

    pub fn parse_expression(&mut self) -> Expression {
        let atom = self.parse_atom();
        let maybe_binary_expression = self.parse_maybe_binary(atom, 0);

        if let Expression::Variable(variable_expression) = &maybe_binary_expression {
            if let Some(expression) = self.parse_maybe_call(&variable_expression) {
                return expression;
            }
        }

        return maybe_binary_expression;
    }

    pub fn parse_maybe_call(&mut self, expression: &VariableExpression) -> Option<Expression> {
        if let TokenValue::OpenParen = self.tokens.peek().unwrap().value {
            self.tokens.by_ref().next();

            let mut call_args: Vec<Expression> = vec![];
            while self.tokens.by_ref().peek().is_some() {
                let possible_end_token = self.tokens.by_ref().peek().expect("Unexpected EOF");
                if let CloseParen = &possible_end_token.value {
                    self.tokens.by_ref().next();
                    break;
                }

                let call_arg_expression = self.parse_expression();
                call_args.push(call_arg_expression);

                let _ = {
                    let delimiter = self.tokens.by_ref().peek().expect("Unexpected EOF");
                    if let Comma = &delimiter.value {
                        self.tokens.by_ref().next()
                    } else if let CloseParen = &delimiter.value {
                        self.tokens.by_ref().next();
                        break;
                    } else {
                        panic!("expected `,`, got {:?}", delimiter.value)
                    }
                };
            }

            return Some(Expression::Call(Box::new(CallExpression::new(
                expression.binding.clone(),
                call_args,
            ))));
        }

        return None;
    }

    pub fn parse_atom(&mut self) -> Expression {
        let peek = self.tokens.peek().expect("Unexpected EOF");
        match &peek.value {
            TokenValue::NumConst(float) => {
                self.tokens.by_ref().next();
                Expression::NumberLiteral(Box::new(NumberLiteralExpression::new(*float)))
            }
            TokenValue::Identifier(identifier) => {
                self.tokens.by_ref().next();
                Expression::Variable(Box::new(VariableExpression::new(identifier.clone())))
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
        } else {
            panic!("Expected token Keyword(Do), got {:?}", open_token)
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

                let peek = self.tokens.by_ref().peek().expect("Unexpected EOF");
                let args = if let TokenValue::OpenParen = peek.value {
                    self.parse_function_args()
                } else {
                    Vec::new()
                };

                FunctionExpression::new(name, self.parse_block(), args)
            } else {
                panic!("Unexpected token, expected Identifier ")
            }
        } else {
            panic!("Unexpected token, expected Keyword `fn` ")
        }
    }

    pub fn parse_extern_function(&mut self) -> ExternFunctionExpression {
        let def_extern_keyword = self.tokens.by_ref().peek().expect("Unexpected EOF");

        if let Keyword(DefExtern) = &def_extern_keyword.value {
            self.tokens.by_ref().next();

            let name_token = self.tokens.by_ref().peek().expect("Unexpected EOF");
            if let Identifier(name) = &name_token.value {
                self.tokens.by_ref().next();

                let peek = self.tokens.by_ref().peek().expect("Unexpected EOF");
                let args = if let TokenValue::OpenParen = peek.value {
                    self.parse_function_args()
                } else {
                    Vec::new()
                };

                ExternFunctionExpression::new(name, args)
            } else {
                panic!("Unexpected token, expected Identifier ")
            }
        } else {
            panic!("Unexpected token, expected Keyword `defextern` ")
        }
    }

    fn parse_function_args(&mut self) -> Vec<FunctionArgument> {
        let open_paren = self.tokens.by_ref().peek().expect("Unexpected EOF");
        if let OpenParen = &open_paren.value {
            self.tokens.by_ref().next();
        } else {
            panic!("expected `(`, got {:?}", open_paren.value)
        }

        let mut args: Vec<FunctionArgument> = vec![];
        while self.tokens.by_ref().peek().is_some() {
            let possible_end_token = self.tokens.by_ref().peek().expect("Unexpected EOF");
            if let CloseParen = &possible_end_token.value {
                self.tokens.by_ref().next();
                break;
            }

            let fn_signature = self.parse_function_arg_signature();
            args.push(fn_signature);

            let _ = {
                let delimiter = self.tokens.by_ref().peek().expect("Unexpected EOF");
                if let Comma = &delimiter.value {
                    self.tokens.by_ref().next()
                } else if let CloseParen = &delimiter.value {
                    self.tokens.by_ref().next();
                    break;
                } else {
                    panic!("expected `,`, got {:?}", delimiter.value)
                }
            };
        }

        args
    }

    fn parse_type_signature(&mut self) -> Type {
        let possible_colon = self.tokens.peek().expect("Unexpected EOF");
        if let TokenValue::Colon = &possible_colon.value {
            self.tokens.by_ref().next();
        } else {
            panic!("Expected :, got {:?}", possible_colon)
        }

        let possible_ident = self.tokens.by_ref().peek().expect("Unexpected EOF");
        if let TokenValue::Identifier(ident) = &possible_ident.value {
            self.tokens.by_ref().next();
            Type::from(ident.clone())
        } else {
            panic!("Expected type-identifier, got {:?}", possible_ident)
        }
    }

    fn parse_function_arg_signature(&mut self) -> FunctionArgument {
        let peek = self.tokens.by_ref().peek().expect("Unexpected EOF");
        match &peek.value {
            Identifier(binding) => {
                self.tokens.by_ref().next();
                let arg_type = self.parse_type_signature();
                FunctionArgument::new(binding.clone(), arg_type)
            }
            token => panic!("Unexpected token {:?}, expected identifier", token),
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
            Keyword(DefExtern) => Some(Expression::ExternFunction(Box::new(
                self.parse_extern_function(),
            ))),
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
