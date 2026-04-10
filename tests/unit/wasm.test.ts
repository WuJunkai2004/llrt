describe("WebAssembly", () => {
  // Minimal WASM binary header: \0asm followed by version 1
  const wasmBuffer = new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0]);

  it("validate() should validate a correct WASM binary", () => {
    expect(WebAssembly.validate(wasmBuffer)).toBe(true);
    expect(WebAssembly.validate(new Uint8Array([0, 0, 0, 0]))).toBe(false);
  });

  it("compile() should compile a WASM binary into a WebAssembly.Module", async () => {
    const module = await WebAssembly.compile(wasmBuffer);
    expect(module).toBeInstanceOf(WebAssembly.Module);
  });

  it("instantiate() should instantiate a WASM binary or module", async () => {
    // Overload 1: bufferSource
    const result = await WebAssembly.instantiate(wasmBuffer);
    expect(result.module).toBeInstanceOf(WebAssembly.Module);
    expect(result.instance).toBeInstanceOf(WebAssembly.Instance);

    // Overload 2: WebAssembly.Module
    const instance = await WebAssembly.instantiate(result.module);
    expect(instance).toBeInstanceOf(WebAssembly.Instance);
  });

  it("compileStreaming() should compile a WASM response stream", async () => {
    const response = Promise.resolve(
      new Response(wasmBuffer, {
        headers: { "Content-Type": "application/wasm" },
      })
    );
    const module = await WebAssembly.compileStreaming(response);
    expect(module).toBeInstanceOf(WebAssembly.Module);
  });

  it("instantiateStreaming() should instantiate a WASM response stream", async () => {
    const response = Promise.resolve(
      new Response(wasmBuffer, {
        headers: { "Content-Type": "application/wasm" },
      })
    );
    const result = await WebAssembly.instantiateStreaming(response);
    expect(result.module).toBeInstanceOf(WebAssembly.Module);
    expect(result.instance).toBeInstanceOf(WebAssembly.Instance);
  });

  it("should execute logic from a compiled WASM file (add, factorial, power)", async () => {
    const fs = await import("node:fs/promises");
    const path = await import("node:path");

    // Resolve the path to the fixtures directory
    const wasmPath = path.join(process.cwd(), "tests/unit/fixtures/test.wasm");
    const compiledWasmBuffer = await fs.readFile(wasmPath);

    const { instance } = await WebAssembly.instantiate(compiledWasmBuffer);
    const exports = instance.exports as any;

    expect(exports.add(5, 7)).toBe(12);
    expect(exports.factorial(5)).toBe(120);
    expect(exports.power(2, 10)).toBe(1024);
    expect(exports.fast_power(2, 10)).toBe(1024);
  });
});
