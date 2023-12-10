# rust-hmr-experiment

Experimenting the compilation of a single `.rs` file into `.wasm` module


Ultimately try to make Proof of Concept for HMR on rust web frontend framework (Leptos)

## Brief Roadmap

- [x] compile whole cargo project into WASM (main module)
- [x] Compile a single rust file into a separate WASM file (component module)
- [ ] Load the component WASM module from main WASM module
  - TODO: `__wbindgen_externref_xform__` field in import-object for WASM instanciation
    - `__wbindgen_externref_table_grow`: `(param i32) (result i32)`
    - `__wbindgen_externref_table_set_null`: `(param i32)`
- [ ] HMR (without Leptos)
- [ ] HMR (with Leptos)
- [ ] rust source modifier plugin (for activating HMR thru plugin interface)
