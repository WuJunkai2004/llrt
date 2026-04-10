// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(non_snake_case)]

use std::sync::OnceLock;
use std::{ffi::c_char, ffi::c_void};

const STACK_SIZE: u32 = 64 * 1024;
const HEAP_SIZE: u32 = 64 * 1024;
const ERROR_BUFFER_SIZE: usize = 256;

static RUNTIME_INIT_OK: OnceLock<bool> = OnceLock::new();

type WasmModule = *mut c_void;
type WasmModuleInstance = *mut c_void;

unsafe extern "C" {
    fn wasm_runtime_init() -> bool;
    fn wasm_runtime_load(
        buffer: *const u8,
        size: u32,
        error_buffer: *mut c_char,
        error_buffer_size: u32,
    ) -> WasmModule;
    fn wasm_runtime_unload(module: WasmModule);
    fn wasm_runtime_instantiate(
        module: WasmModule,
        stack_size: u32,
        heap_size: u32,
        error_buffer: *mut c_char,
        error_buffer_size: u32,
    ) -> WasmModuleInstance;
    fn wasm_runtime_deinstantiate(module_instance: WasmModuleInstance);
}

fn ensure_runtime_init() -> bool {
    *RUNTIME_INIT_OK.get_or_init(|| {
        // SAFETY: Called through OnceLock to ensure one-time runtime initialization.
        unsafe { wasm_runtime_init() }
    })
}

fn load_module(data_ptr: *const u8, data_len: usize) -> Option<WasmModule> {
    if data_ptr.is_null() || data_len == 0 {
        return None;
    }

    let data_len = u32::try_from(data_len).ok()?;
    let mut error_buffer = [0_i8; ERROR_BUFFER_SIZE];

    // SAFETY: The pointer/size are checked above and originate from caller-provided wasm bytes.
    let module = unsafe {
        wasm_runtime_load(
            data_ptr,
            data_len,
            error_buffer.as_mut_ptr(),
            ERROR_BUFFER_SIZE as u32,
        )
    };

    if module.is_null() {
        None
    } else {
        Some(module)
    }
}

fn validate_inner(data_ptr: *const u8, data_len: usize) -> bool {
    if !ensure_runtime_init() {
        return false;
    }

    let Some(module) = load_module(data_ptr, data_len) else {
        return false;
    };

    // SAFETY: module was returned by wasm_runtime_load and must be released once.
    unsafe {
        wasm_runtime_unload(module);
    }

    true
}

fn compile_inner(data_ptr: *const u8, data_len: usize) -> bool {
    validate_inner(data_ptr, data_len)
}

fn instantiate_inner(data_ptr: *const u8, data_len: usize) -> bool {
    if !ensure_runtime_init() {
        return false;
    }

    let Some(module) = load_module(data_ptr, data_len) else {
        return false;
    };

    let mut error_buffer = [0_i8; ERROR_BUFFER_SIZE];
    // SAFETY: module is a valid handle from wasm_runtime_load.
    let module_instance = unsafe {
        wasm_runtime_instantiate(
            module,
            STACK_SIZE,
            HEAP_SIZE,
            error_buffer.as_mut_ptr(),
            ERROR_BUFFER_SIZE as u32,
        )
    };

    if module_instance.is_null() {
        // SAFETY: module was loaded successfully and must be unloaded even on instantiate failure.
        unsafe {
            wasm_runtime_unload(module);
        }
        return false;
    }

    // SAFETY: module_instance/module are valid runtime handles and released exactly once.
    unsafe {
        wasm_runtime_deinstantiate(module_instance);
        wasm_runtime_unload(module);
    }

    true
}

fn compile_streaming_inner(data_ptr: *const u8, data_len: usize) -> bool {
    compile_inner(data_ptr, data_len)
}

fn instantiate_streaming_inner(data_ptr: *const u8, data_len: usize) -> bool {
    instantiate_inner(data_ptr, data_len)
}

#[no_mangle]
pub extern "C" fn llrt_wasm_validate(data_ptr: *const u8, data_len: usize) -> i32 {
    i32::from(validate_inner(data_ptr, data_len))
}

#[no_mangle]
pub extern "C" fn llrt_wasm_compile(data_ptr: *const u8, data_len: usize) -> i32 {
    i32::from(compile_inner(data_ptr, data_len))
}

#[no_mangle]
pub extern "C" fn llrt_wasm_instantiate(data_ptr: *const u8, data_len: usize) -> i32 {
    i32::from(instantiate_inner(data_ptr, data_len))
}

#[no_mangle]
pub extern "C" fn llrt_wasm_compile_streaming(data_ptr: *const u8, data_len: usize) -> i32 {
    i32::from(compile_streaming_inner(data_ptr, data_len))
}

#[no_mangle]
pub extern "C" fn llrt_wasm_instantiate_streaming(data_ptr: *const u8, data_len: usize) -> i32 {
    i32::from(instantiate_streaming_inner(data_ptr, data_len))
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_EMPTY_WASM: [u8; 8] = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    const INVALID_WASM: [u8; 4] = [0x00, 0x61, 0x73, 0x00];

    #[test]
    fn validate_symbol_handles_valid_and_invalid_inputs() {
        assert_eq!(
            llrt_wasm_validate(VALID_EMPTY_WASM.as_ptr(), VALID_EMPTY_WASM.len()),
            1
        );
        assert_eq!(llrt_wasm_validate(INVALID_WASM.as_ptr(), INVALID_WASM.len()), 0);
    }

    #[test]
    fn compile_symbol_handles_valid_and_invalid_inputs() {
        assert_eq!(
            llrt_wasm_compile(VALID_EMPTY_WASM.as_ptr(), VALID_EMPTY_WASM.len()),
            1
        );
        assert_eq!(llrt_wasm_compile(INVALID_WASM.as_ptr(), INVALID_WASM.len()), 0);
    }

    #[test]
    fn instantiate_symbol_handles_valid_and_invalid_inputs() {
        assert_eq!(
            llrt_wasm_instantiate(VALID_EMPTY_WASM.as_ptr(), VALID_EMPTY_WASM.len()),
            1
        );
        assert_eq!(
            llrt_wasm_instantiate(INVALID_WASM.as_ptr(), INVALID_WASM.len()),
            0
        );
    }

    #[test]
    fn compile_streaming_symbol_handles_valid_and_invalid_inputs() {
        assert_eq!(
            llrt_wasm_compile_streaming(VALID_EMPTY_WASM.as_ptr(), VALID_EMPTY_WASM.len()),
            1
        );
        assert_eq!(
            llrt_wasm_compile_streaming(INVALID_WASM.as_ptr(), INVALID_WASM.len()),
            0
        );
    }

    #[test]
    fn instantiate_streaming_symbol_handles_valid_and_invalid_inputs() {
        assert_eq!(
            llrt_wasm_instantiate_streaming(VALID_EMPTY_WASM.as_ptr(), VALID_EMPTY_WASM.len()),
            1
        );
        assert_eq!(
            llrt_wasm_instantiate_streaming(INVALID_WASM.as_ptr(), INVALID_WASM.len()),
            0
        );
    }
}
