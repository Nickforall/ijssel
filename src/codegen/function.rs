use crate::parser::parser::FunctionExpression;
use crate::raw_cstr;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub fn compile_function(module: &LLVMModuleRef, expression: &FunctionExpression) {
    let function_type = unsafe {
        let mut arg_type = [];
        let fn_type = LLVMFunctionType(
            LLVMInt32Type(),
            arg_type.as_mut_ptr(),
            arg_type.len() as u32,
            0,
        );

        fn_type
    };

    let function_name = expression.name.as_str();
    let function = unsafe { LLVMAddFunction(*module, raw_cstr(function_name), function_type) };
    let block = super::block::compile_block(module, &expression.body);

    unsafe { LLVMAppendExistingBasicBlock(function, block) }
}
