use super::ModuleFunction;
use crate::parser::{AstModule, Expression};
use std::collections::HashMap;

pub struct Module {
    pub functions: HashMap<String, ModuleFunction>,
    pub name: String,
}

impl Module {
    pub fn from_ast_module(module: AstModule) -> Self {
        let mut functions = HashMap::new();

        for expression in module.expressions {
            let (name, expr) = match expression {
                Expression::Function(ref internal) => {
                    (internal.name.clone(), ModuleFunction::from(&expression))
                }
                Expression::ExternFunction(ref internal) => {
                    (internal.name.clone(), ModuleFunction::from(&expression))
                }
                _ => panic!("Invalid top-level expression"),
            };

            functions.insert(name, expr);
        }

        Module {
            functions,
            name: module.name,
        }
    }
}
