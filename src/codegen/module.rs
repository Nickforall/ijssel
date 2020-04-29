use crate::parser::parser::{Expression, Module};

use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub fn compile_module(module: &Module) -> LLVMModuleRef {
    let module_name = crate::raw_cstr(module.name.as_str());
    let llvm_module = unsafe { LLVMModuleCreateWithName(module_name) };

    for expression in &module.expressions {
        match expression {
            Expression::Function(expr) => super::function::compile_function(&llvm_module, &**expr),
            expr => panic!("Incorrect module level expression {:?}", expr),
        }
    }

    return llvm_module;
}
