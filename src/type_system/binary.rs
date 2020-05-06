use super::InferType;
use crate::parser::{BinaryExpression, NumberLiteralExpression, Type};

impl InferType for NumberLiteralExpression {
    fn infer_type(&self) -> Option<Type> {
        Some(self.return_type.clone())
    }
}

impl InferType for BinaryExpression {
    fn infer_type(&self) -> Option<Type> {
        let right_type = self.right.infer_type();
        let left_type = self.left.infer_type();

        if left_type != right_type {
            panic!("Incompatible types")
        }

        left_type
    }
}
