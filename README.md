# binaryen-2716

This repo is a there to reproduce issue #2716 in binaryen.

Make sure you have wasm-pack and wasm2js in your PATH.

# Build the library
```bash
wasm-pack build
```

# wasm2js
```
cd pkg/
wasm2js binaryen_2716_bg.wasm -o test.js
```