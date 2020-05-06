use crate::parser::Expression;

pub use crate::parser::Type;

mod binary;
mod block;
#[cfg(test)]
mod tests;

trait InferType {
    fn infer_type(&self) -> Option<Type>;
}

impl InferType for Expression {
    fn infer_type(&self) -> Option<Type> {
        match self {
            Expression::NumberLiteral(expr) => expr.infer_type(),
            Expression::Binary(expr) => expr.infer_type(),
            _ => unimplemented!(),
        }
    }
}
