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
            TokenValue::OpenSqBracket => Expression::Array(Box::new(self.parse_array_expression())),
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

    pub fn parse_block(&mut self, return_type: Type) -> BlockExpression {
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

        return BlockExpression::new(expressions, return_type);
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

                let ret_type = self.parse_return_type_signature(Type::Unknown);
                FunctionExpression::new(name, self.parse_block(ret_type), args)
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

                let ret_type = self.parse_return_type_signature(Type::Void);

                ExternFunctionExpression::new(name, args, ret_type)
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

    fn parse_return_type_signature(&mut self, default: Type) -> Type {
        let possible_arrow = self.tokens.peek();
        if let Some(token) = &possible_arrow {
            if token.value == TokenValue::ReturnArrow {
                self.tokens.by_ref().next();
            } else {
                return default;
            }
        } else {
            return default;
        }

        self.parse_type()
    }

    fn parse_type_signature(&mut self) -> Type {
        let possible_colon = self.tokens.peek().expect("Unexpected EOF");
        if let TokenValue::Colon = &possible_colon.value {
            self.tokens.by_ref().next();
        } else {
            panic!("Expected :, got {:?}", possible_colon)
        }

        self.parse_type()
    }

    fn parse_type(&mut self) -> Type {
        let peek = self.tokens.by_ref().peek().expect("Unexpected EOF");
        if let TokenValue::Identifier(ident) = &peek.value {
            self.tokens.by_ref().next();
            Type::from(ident.clone())
        } else if let TokenValue::OpenSqBracket = &peek.value {
            self.tokens.by_ref().next();
            let inner_type = self.parse_type();

            let peek = self.tokens.by_ref().peek().expect("Unexpected EOF");
            if peek.value != TokenValue::SemiColon {
                panic!(
                    "Expected length definition seperator of list-type `;`, got {:?}",
                    peek
                )
            }

            self.tokens.by_ref().next();

            let peek = self.tokens.by_ref().peek().expect("Unexpected EOF");
            if let TokenValue::NumConst(length) = &peek.value {
                self.tokens.by_ref().next();

                let peek = self.tokens.by_ref().peek().expect("Unexpected EOF");
                if peek.value != TokenValue::CloseSqBracket {
                    panic!("Expected closing bracket of list-type `]`, got {:?}", peek)
                }
                self.tokens.by_ref().next();

                Type::ListOf(Box::new(inner_type), *length as u32)
            } else {
                panic!("Expected list length definition, got {:?}", peek)
            }
        } else {
            panic!("Expected type-identifier, got {:?}", peek)
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

    fn parse_array_expression(&mut self) -> ArrayExpression {
        let peek = self.tokens.peek().unwrap();
        if let TokenValue::OpenSqBracket = &peek.value {
            self.tokens.by_ref().next();

            let mut items: Vec<Expression> = vec![];
            while self.tokens.by_ref().peek().is_some() {
                let possible_end_token = self.tokens.by_ref().peek().expect("Unexpected EOF");
                if let TokenValue::CloseSqBracket = &possible_end_token.value {
                    self.tokens.by_ref().next();
                    break;
                }

                let item_expression = self.parse_expression();
                items.push(item_expression);

                let _ = {
                    let delimiter = self.tokens.by_ref().peek().expect("Unexpected EOF");
                    if let Comma = &delimiter.value {
                        self.tokens.by_ref().next()
                    } else if let TokenValue::CloseSqBracket = &delimiter.value {
                        self.tokens.by_ref().next();
                        break;
                    } else {
                        panic!("expected `,`, got {:?}", delimiter.value)
                    }
                };
            }

            ArrayExpression::new(items)
        } else {
            panic!("Expected [, got {:?}",)
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
