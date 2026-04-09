// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use std::env;

use libloading::{Library, Symbol};
use llrt_utils::bytes::ObjectBytes;
use rquickjs::{function::Func, Ctx, Exception, Object, Result};

type WasmBridgeFn = extern "C" fn(*const u8, usize) -> i32;

const LIB_PATH_ENV: &str = "LLRT_WEBASSEMBLY_LIB_PATH";
const DEFAULT_LIB_NAME: &str = "llrt-WebAssembly.so";
const DEFAULT_FALLBACK_LIB_NAME: &str = "libllrt_WebAssembly.so";

fn resolve_library_paths() -> Vec<String> {
    let mut paths = Vec::with_capacity(4);

    if let Ok(path) = env::var(LIB_PATH_ENV) {
        paths.push(path);
    }

    paths.push(format!("./{DEFAULT_LIB_NAME}"));
    paths.push(DEFAULT_LIB_NAME.to_string());
    paths.push(DEFAULT_FALLBACK_LIB_NAME.to_string());
    paths
}

fn call_bridge<'js>(
    ctx: &Ctx<'js>,
    bytes: &[u8],
    symbol_name: &'static [u8],
    operation: &str,
) -> Result<bool> {
    let mut last_error = String::new();

    for path in resolve_library_paths() {
        // SAFETY: Path is controlled by runtime configuration and used only for loading shared library.
        let library = match unsafe { Library::new(&path) } {
            Ok(lib) => lib,
            Err(err) => {
                last_error = err.to_string();
                continue;
            },
        };

        // SAFETY: Symbol is resolved from the just-opened library and invoked immediately.
        let symbol: Symbol<WasmBridgeFn> = match unsafe { library.get(symbol_name) } {
            Ok(sym) => sym,
            Err(err) => {
                last_error = err.to_string();
                continue;
            },
        };

        let status = symbol(bytes.as_ptr(), bytes.len());
        return Ok(status == 1);
    }

    Err(Exception::throw_message(
        ctx,
        &format!("Failed to load {operation} bridge from {DEFAULT_LIB_NAME}: {last_error}"),
    ))
}

fn compile_like<'js>(
    ctx: Ctx<'js>,
    source: ObjectBytes<'js>,
    symbol_name: &'static [u8],
    operation: &str,
) -> Result<Object<'js>> {
    let bytes = source.into_bytes(&ctx)?;
    let ok = call_bridge(&ctx, &bytes, symbol_name, operation)?;

    if !ok {
        return Err(Exception::throw_type(
            &ctx,
            &format!("WebAssembly {operation} failed"),
        ));
    }

    let module = Object::new(ctx.clone())?;
    module.set("__llrtWebAssemblyModule", true)?;
    Ok(module)
}

fn validate<'js>(ctx: Ctx<'js>, source: ObjectBytes<'js>) -> Result<bool> {
    let bytes = source.into_bytes(&ctx)?;
    call_bridge(&ctx, &bytes, b"llrt_wasm_validate\0", "validate")
}

fn compile<'js>(ctx: Ctx<'js>, source: ObjectBytes<'js>) -> Result<Object<'js>> {
    compile_like(ctx, source, b"llrt_wasm_compile\0", "compile")
}

fn instantiate<'js>(ctx: Ctx<'js>, source: ObjectBytes<'js>) -> Result<Object<'js>> {
    let bytes = source.into_bytes(&ctx)?;
    let ok = call_bridge(&ctx, &bytes, b"llrt_wasm_instantiate\0", "instantiate")?;

    if !ok {
        return Err(Exception::throw_type(&ctx, "WebAssembly instantiate failed"));
    }

    let result = Object::new(ctx.clone())?;
    let module = Object::new(ctx.clone())?;
    module.set("__llrtWebAssemblyModule", true)?;
    let instance = Object::new(ctx.clone())?;
    result.set("module", module)?;
    result.set("instance", instance)?;
    Ok(result)
}

fn compile_streaming<'js>(ctx: Ctx<'js>, source: ObjectBytes<'js>) -> Result<Object<'js>> {
    compile_like(
        ctx,
        source,
        b"llrt_wasm_compile_streaming\0",
        "compileStreaming",
    )
}

fn instantiate_streaming<'js>(ctx: Ctx<'js>, source: ObjectBytes<'js>) -> Result<Object<'js>> {
    let bytes = source.into_bytes(&ctx)?;
    let ok = call_bridge(
        &ctx,
        &bytes,
        b"llrt_wasm_instantiate_streaming\0",
        "instantiateStreaming",
    )?;

    if !ok {
        return Err(Exception::throw_type(
            &ctx,
            "WebAssembly instantiateStreaming failed",
        ));
    }

    let result = Object::new(ctx.clone())?;
    let module = Object::new(ctx.clone())?;
    module.set("__llrtWebAssemblyModule", true)?;
    let instance = Object::new(ctx.clone())?;
    result.set("module", module)?;
    result.set("instance", instance)?;
    Ok(result)
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let webassembly = Object::new(ctx.clone())?;

    webassembly.set("compile", Func::from(compile))?;
    webassembly.set("validate", Func::from(validate))?;
    webassembly.set("instantiate", Func::from(instantiate))?;
    webassembly.set("compileStreaming", Func::from(compile_streaming))?;
    webassembly.set("instantiateStreaming", Func::from(instantiate_streaming))?;

    ctx.globals().set("WebAssembly", webassembly)?;
    Ok(())
}
