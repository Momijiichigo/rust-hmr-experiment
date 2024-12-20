# rust-hmr-experiment

Rust Web Dev;
Proof of Concept for HMR (Hot Module Replacement) using WASM modules.

![overview](./README_MEDIA/brief_overview.png)

## File tree
- `./hmr-server`
  - It does:
    - compilation & modification of host-wasm
    - compilation & modification of mod1.wasm
    - serves to `localhost:3000`
- `./wasm-project`
  - It is the sample project that will be compiled into wasm
  - contains `lib.rs` and `mod1.rs`

## Run
```sh
cd hmr-server
cargo run
```

## Brief Roadmap

- [x] compile whole cargo project into WASM (main module)
- [x] modify the main wasm so it exports all the library functions
  - e.g. `alloc::alloc::handle_alloc_error`
- [x] Compile a single rust file into a separate WASM file (component module)
- [x] Load the component WASM module (`mod1.wasm`) from main WASM module
  - [x] Import object: `__wbindgen_placeholder__`, `__wbindgen_externref_xform__` fields
  - [x] Use of [Proxy](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy) in JS to flexibly resolve the importObject for WASM instanciation
  - [x] Compile `mod1.rs` into object file with `--emit obj` flag in `rustc`
    - `mod1.wasm` will be minimal and imports all library functions at runtime
    - also imports memory from outside
  - [x] Modify the wasm binary with [Walrus](https://github.com/rustwasm/walrus)
    - [x] add exports
      - [x] parse Custom Linking section of wasm binary and obtain name map of functions
    - [x] demangle import & func names
  - [x] pass in the host's memory & imports to instanciate `mod1.wasm`
- [ ] Error: accessing `thread_local!` value from `mod1.wasm` fails; accessing to wrong memory address
  - Hmmmm
  - Try: Override `thread_local` crate for the compilation
    - Try it in the host module compilation first
- [ ] Modify the source code and pass to the compiling process
  - Idea: Virtual sandboxed filesystem that returns processed file content
    - Similar idea as using `Proxy` in JS but for filesystems

  - [ ] substitute `crate::mod` with `proj_name::mod` only in the module compilation
- [ ] rust source modifier plugin (for activating HMR thru plugin interface)

## Current Status

![image](https://github.com/user-attachments/assets/6354ee3d-374c-45cb-810c-8dadd8e12836)
