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
});
