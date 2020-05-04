use crate::parser::{ExternFunctionExpression, FunctionExpression};
use crate::raw_cstr;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;

pub fn compile_function(module: &LLVMModuleRef, expression: &FunctionExpression) {
    let args_count = expression.arguments.len();
    let mut args: Vec<LLVMTypeRef> = Vec::with_capacity(args_count);
    let arg_type: *mut *mut llvm_sys::LLVMType = {
        for _arg in &expression.arguments {
            // TODO: types
            args.push(unsafe { LLVMInt32Type() });
        }

        args.as_mut_ptr()
    };

    let function_type = unsafe {
        let fn_type = LLVMFunctionType(LLVMInt32Type(), arg_type, args.len() as u32, 0);
        fn_type
    };

    let function_name = expression.name.as_str();
    let function = unsafe { LLVMAddFunction(*module, raw_cstr(function_name), function_type) };

    let mut function_bindings: super::block::BindingsHashMap = HashMap::new();

    for (i, item) in expression.arguments.clone().into_iter().enumerate() {
        let binding = raw_cstr(item.binding_name.as_str());
        let value_ref = unsafe {
            let value = LLVMGetParam(function, i as u32);
            LLVMSetValueName2(value, binding, item.binding_name.len());
            value
        };

        function_bindings.insert(item.binding_name, value_ref);
    }

    let block = super::block::compile_block(module, &expression.body, function_bindings);

    unsafe { LLVMAppendExistingBasicBlock(function, block) }
}

pub fn compile_extern_function(module: &LLVMModuleRef, expression: &ExternFunctionExpression) {
    let args_count = expression.arguments.len();
    let mut args: Vec<LLVMTypeRef> = Vec::with_capacity(args_count);
    let arg_type: *mut *mut llvm_sys::LLVMType = {
        for _arg in &expression.arguments {
            // TODO: types
            args.push(unsafe { LLVMInt32Type() });
        }

        args.as_mut_ptr()
    };

    let function_type = unsafe {
        let fn_type = LLVMFunctionType(LLVMInt32Type(), arg_type, args.len() as u32, 0);
        fn_type
    };

    let function_name = expression.name.as_str();
    let function = unsafe { LLVMAddFunction(*module, raw_cstr(function_name), function_type) };

    unsafe { LLVMSetLinkage(function, llvm_sys::LLVMLinkage::LLVMExternalLinkage) }
}
