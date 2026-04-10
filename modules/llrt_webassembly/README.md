# llrt_webassembly

Provides the global `WebAssembly` object for LLRT by loading bridge symbols from `llrt-WebAssembly.so`.

## Linux PoC build

Build the shared runtime bridge:

```bash
make webassembly-runtime
```

By default, LLRT resolves the runtime bridge from one of:

- `./llrt-WebAssembly.so`
- `llrt-WebAssembly.so`
- `libllrt_WebAssembly.so`
- `libllrt_webassembly.so`

You can also override the path with:

```bash
export LLRT_WEBASSEMBLY_LIB_PATH=/absolute/path/to/llrt-WebAssembly.so
```
