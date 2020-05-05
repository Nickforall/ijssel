use crate::parser::{Expression, ExternFunctionExpression, FunctionExpression};
use std::convert::From;

pub enum ModuleFunction {
    Local(FunctionExpression),
    External(ExternFunctionExpression),
}

impl From<&Expression> for ModuleFunction {
    fn from(value: &Expression) -> Self {
        match value {
            Expression::Function(expr) => ModuleFunction::Local(expr.as_ref().clone()),
            Expression::ExternFunction(expr) => ModuleFunction::External(expr.as_ref().clone()),
            _ => panic!("Invalid top level expression"),
        }
    }
}
