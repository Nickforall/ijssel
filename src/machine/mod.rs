use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;

use llvm_sys::target_machine::*;

use crate::raw_cstr;

pub fn compile_to_current_target(
    module: &LLVMModuleRef,
    output_file: &str,
    file_type: LLVMCodeGenFileType,
    debug: bool,
) {
    let triple = unsafe { LLVMGetDefaultTargetTriple() };

    unsafe {
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmParsers();
        LLVM_InitializeAllAsmPrinters();
    }

    let mut error: *mut libc::c_char = std::ptr::null_mut();
    let mut target: LLVMTargetRef = std::ptr::null_mut();

    unsafe { LLVMGetTargetFromTriple(triple, &mut target, &mut error) };

    if target.is_null() {
        panic!("No target, error: {:?}", error);
    }

    let optimisation_level = if debug {
        LLVMCodeGenOptLevel::LLVMCodeGenLevelNone
    } else {
        LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive
    };

    let target_machine = unsafe {
        LLVMCreateTargetMachine(
            target,
            triple,
            raw_cstr("generic"),
            raw_cstr(""),
            optimisation_level,
            LLVMRelocMode::LLVMRelocDefault,
            LLVMCodeModel::LLVMCodeModelDefault,
        )
    };

    let mut error: *mut libc::c_char = std::ptr::null_mut();

    unsafe {
        LLVMSetModuleDataLayout(*module, LLVMCreateTargetDataLayout(target_machine));
        LLVMSetTarget(*module, triple);

        let file = std::ffi::CString::new(output_file).unwrap().into_raw();

        LLVMTargetMachineEmitToFile(
            target_machine,
            *module,
            file as *mut i8,
            file_type,
            &mut error,
        );
    };
}
