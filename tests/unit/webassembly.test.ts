const VALID_EMPTY_WASM = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
]);
const INVALID_WASM = new Uint8Array([0x00, 0x61, 0x73, 0x00]);

const isBridgeMissing = (() => {
  try {
    globalThis.WebAssembly.validate(VALID_EMPTY_WASM);
    return false;
  } catch (error) {
    return String(error).includes("Failed to load bridge");
  }
})();

describe("WebAssembly global", () => {
  it("should expose WebAssembly on globalThis", () => {
    expect(globalThis.WebAssembly).toBeDefined();
  });

  it("should expose required bridge functions", () => {
    expect(globalThis.WebAssembly.compile).toBeDefined();
    expect(globalThis.WebAssembly.validate).toBeDefined();
    expect(globalThis.WebAssembly.instantiate).toBeDefined();
    expect(globalThis.WebAssembly.compileStreaming).toBeDefined();
    expect(globalThis.WebAssembly.instantiateStreaming).toBeDefined();
  });

  it("should return load error when bridge library is unavailable", () => {
    if (!isBridgeMissing) {
      return;
    }

    expect(() => globalThis.WebAssembly.validate(VALID_EMPTY_WASM)).toThrow(
      /Failed to load bridge/
    );
    expect(() => globalThis.WebAssembly.compile(VALID_EMPTY_WASM)).toThrow(
      /Failed to load bridge/
    );
    expect(() => globalThis.WebAssembly.instantiate(VALID_EMPTY_WASM)).toThrow(
      /Failed to load bridge/
    );
    expect(() =>
      globalThis.WebAssembly.compileStreaming(VALID_EMPTY_WASM)
    ).toThrow(/Failed to load bridge/);
    expect(() =>
      globalThis.WebAssembly.instantiateStreaming(VALID_EMPTY_WASM)
    ).toThrow(/Failed to load bridge/);
  });

  it("should validate/compile/instantiate with valid and invalid wasm bytes when bridge is available", () => {
    if (isBridgeMissing) {
      return;
    }

    expect(globalThis.WebAssembly.validate(VALID_EMPTY_WASM)).toEqual(true);
    expect(globalThis.WebAssembly.validate(INVALID_WASM)).toEqual(false);

    const compiled = globalThis.WebAssembly.compile(VALID_EMPTY_WASM);
    expect(compiled.__llrtWebAssemblyModule).toEqual(true);
    expect(() => globalThis.WebAssembly.compile(INVALID_WASM)).toThrow(
      /WebAssembly compile failed/
    );

    const instantiated = globalThis.WebAssembly.instantiate(VALID_EMPTY_WASM);
    expect(instantiated.module.__llrtWebAssemblyModule).toEqual(true);
    expect(instantiated.instance).toBeDefined();
    expect(() => globalThis.WebAssembly.instantiate(INVALID_WASM)).toThrow(
      /WebAssembly instantiate failed/
    );
  });

  it("should support compileStreaming/instantiateStreaming behavior when bridge is available", () => {
    if (isBridgeMissing) {
      return;
    }

    const compiled = globalThis.WebAssembly.compileStreaming(VALID_EMPTY_WASM);
    expect(compiled.__llrtWebAssemblyModule).toEqual(true);
    expect(() =>
      globalThis.WebAssembly.compileStreaming(INVALID_WASM)
    ).toThrow(/WebAssembly compileStreaming failed/);

    const instantiated =
      globalThis.WebAssembly.instantiateStreaming(VALID_EMPTY_WASM);
    expect(instantiated.module.__llrtWebAssemblyModule).toEqual(true);
    expect(instantiated.instance).toBeDefined();
    expect(() =>
      globalThis.WebAssembly.instantiateStreaming(INVALID_WASM)
    ).toThrow(/WebAssembly instantiateStreaming failed/);
  });
});
