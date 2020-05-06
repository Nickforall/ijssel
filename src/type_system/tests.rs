use super::{InferType, Type};

#[test]
fn numeric_literal_type() {
    use crate::parser::NumberLiteralExpression;

    assert_eq!(
        NumberLiteralExpression::new(12.0).infer_type(),
        Some(Type::Int32)
    );
}

#[test]
fn binary_simple_numeric() {
    use crate::parser::{
        tokenizer::BinaryOperator, BinaryExpression, Expression::NumberLiteral,
        NumberLiteralExpression,
    };

    let expr = BinaryExpression::new(
        BinaryOperator::Mul,
        NumberLiteral(Box::new(NumberLiteralExpression::new(12.0))),
        NumberLiteral(Box::new(NumberLiteralExpression::new(12.0))),
    );

    assert_eq!(expr.infer_type(), Some(Type::Int32));
}

#[test]
fn binary_nested_numeric() {
    use crate::parser::{
        tokenizer::BinaryOperator,
        BinaryExpression,
        Expression::{Binary, NumberLiteral},
        NumberLiteralExpression,
    };

    let left_expr = BinaryExpression::new(
        BinaryOperator::Mul,
        NumberLiteral(Box::new(NumberLiteralExpression::new(12.0))),
        NumberLiteral(Box::new(NumberLiteralExpression::new(20.0))),
    );

    let expr = BinaryExpression::new(
        BinaryOperator::Add,
        Binary(Box::new(left_expr)),
        NumberLiteral(Box::new(NumberLiteralExpression::new(19.0))),
    );

    assert_eq!(expr.infer_type(), Some(Type::Int32));
}
