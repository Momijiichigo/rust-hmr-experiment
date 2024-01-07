use anyhow::Context;
use walrus::{IdsToIndices, Module};
pub mod modify_host_wasm;

pub fn module_from_bytes(bytes: &[u8]) -> anyhow::Result<Module> {
    walrus::ModuleConfig::new()
        .parse(bytes)
        .context("failed to parse bytes as wasm")
}

/// renames the function names and import names to the demangled names.
pub fn demangle_funcs(module: &mut Module) {
    for func in module.funcs.iter_mut() {
        let name = match &func.name {
            Some(name) => name,
            None => continue,
        };
        if let Ok(sym) = rustc_demangle::try_demangle(name) {
            func.name = Some(sym.to_string());
        }
    }
}

pub fn demangle_imports(module: &mut Module) {
    for import in module.imports.iter_mut() {
        if let Ok(sym) = rustc_demangle::try_demangle(&import.name) {
            import.name = sym.to_string();
        }
    }

}
