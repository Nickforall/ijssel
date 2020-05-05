use crate::modules::{Module, ModuleFunction};

use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub fn compile_module(module: &Module) -> LLVMModuleRef {
    let module_name = crate::raw_cstr(module.name.as_str());
    let llvm_module = unsafe { LLVMModuleCreateWithName(module_name) };

    for (_, func) in &module.functions {
        match func {
            ModuleFunction::Local(expr) => super::function::compile_function(&llvm_module, &expr),
            ModuleFunction::External(expr) => {
                super::function::compile_extern_function(&llvm_module, &expr)
            }
        }
    }

    return llvm_module;
}
