use llvm_sys::core::*;
use llvm_sys::prelude::*;

use crate::parser::parser::CallExpression;
use crate::raw_cstr;

pub fn compile_call(block: &super::CodegenBlock, expression: &CallExpression) -> LLVMValueRef {
    let name = expression.function_name.as_str();
    let callee = unsafe { LLVMGetNamedFunction(block.module, raw_cstr(name)) };
    if callee.is_null() {
        panic!("Function {} not found", name)
    }

    let expected_args_size = unsafe { LLVMCountParams(callee) } as usize;
    if expected_args_size != expression.args.len() {
        panic!(
            "Expected {} args for function {}, got {}",
            expected_args_size,
            name,
            expression.args.len()
        )
    }

    let mut args: Vec<LLVMValueRef> = Vec::with_capacity(expected_args_size);
    let arg_list: *mut LLVMValueRef = {
        for expression in &expression.args {
            args.push(super::compile_expression_to_instruction(block, expression));
        }

        args.as_mut_ptr()
    };

    let call_value_ref = unsafe {
        LLVMBuildCall(
            block.builder,
            callee,
            arg_list,
            args.len() as u32,
            raw_cstr("__ijssel_tmp"),
        )
    };

    call_value_ref
}
