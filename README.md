# rust-hmr-experiment

Rust Web Dev;
Proof of Concept for HMR (Hot Module Replacement) using WASM modules.

![overview](./README_MEDIA/brief_overview.png)

## Brief Roadmap

- [x] compile whole cargo project into WASM (main module)
- [x] Compile a single rust file into a separate WASM file (component module)
- [ ] Load the component WASM module (`mod1.wasm`) from main WASM module
  - [x] Import object: `__wbindgen_placeholder__`, `__wbindgen_externref_xform__` fields
  - [x] Use of [Proxy](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy) in JS to flexibly resolve the importObject for WASM instanciation
  - [ ] **Obstacle**: `getStringFromWasm0` refers the address of wrong wasm memory (the host wasm memory)
- Workarounds
  - Plan A: runs fast, more efficient
    - [x] Compile `mod1.rs` into object file with `--emit obj` flag in `rustc`
      - `mod1.wasm` will be minimal and imports all library functions at runtime
    - [x] Modify the wasm binary with [Walrus](https://github.com/rustwasm/walrus)
      - [x] add exports
        - [x] parse Custom Linking section of wasm binary and obtain name map of functions
      - [x] demangle import & func names
    - [ ] pass in the host's memory & imports to instanciate `mod1.wasm`
      - **Obstacle**: requires a way to get a list of all functions to be exported from the host wasm
          - the list could be obtained by parsing `rlib` file in `target/../deps` but spec is not documented
      - [ ] embed the (byte_index, func_name) map into the wasm binary by placing `static` map item in `lib.rs` 
        - so the hmr server program can:
        - 1. parse & read the map from generated wasm binary
        - 2. modify the (host) wasm binary to make it export core functions with appropriate names
        - **Obstacle**: Need to make `*const ()` thread safe in order to make it `static` value
  - Plan B: easier to implement (?)
    - generate separate js glue code using [wasm-bindgen-cli-support](https://docs.rs/wasm-bindgen-cli-support/latest/wasm_bindgen_cli_support/index.html)
    - or just make the copy of the glue code for host wasm
    - 
- [ ] HMR (without Leptos)
- [ ] HMR (with Leptos)
- [ ] rust source modifier plugin (for activating HMR thru plugin interface)
