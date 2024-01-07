
import init, { get_wasm_table, get_wasm_memory } from './pkg/wasm_project.js';

const hmr_import_obj = {}
window.__get_hmr_import_obj = () => hmr_import_obj;
/*
  (import "env" "__linear_memory") 1)
  (import "env" "__rust_alloc") (param i32 i32) (result i32))
  (import "env" "__rust_dealloc") (param i32 i32 i32))
  (import "env" "wasm_bindgen::__rt::link_mem_intrinsics::h61974b3836908c38"))
  (import "env" "<str as wasm_bindgen::describe::WasmDescribe>::describe::h63f15c8cb56b70b3"))
  (import "env" "<() as wasm_bindgen::describe::WasmDescribe>::describe::h926585b83620374c"))
  (import "env" "alloc::alloc::handle_alloc_error::h2edda2bcb5c36866") (param i32 i32))


*/

async function run() {
  window.project = await init();
  window.wasmTable = get_wasm_table();
  window.wasmMemory = get_wasm_memory();
  window.imports = __wbg_get_imports();
  hmr_import_obj.__wbindgen_placeholder__ = imports.wbg;
  hmr_import_obj.__wbindgen_externref_xform__ = {
    __wbindgen_externref_table_grow: (delta) => wasmTable.grow(delta),
    __wbindgen_externref_table_set_null: (idx) => wasmTable.set(idx),
  }

  hmr_import_obj.env = new Proxy(
    project, {
      get(target, prop, _receiver) {
	if (prop in target) {
      	  return Reflect.get(...arguments);
      	}
	if (prop === "__linear_memory") {
	  return wasmMemory;
	}
	if (prop.includes("::describe::")) {
	  return () => {};
	}
	if (prop.indexOf("wasm_bindgen::__rt::link_mem_intrinsics") === 0) {
	  return () => {};
	}	
      }
    }
  )

  hmr_import_obj.__wbindgen_placeholder__ = new Proxy(imports.wbg, {
    get(target, prop, _receiver) {
      if (prop in target) {
        return Reflect.get(...arguments);
      }
      if (prop === "__wbindgen_describe") {
	      return () => {};
      }
      const pkgName = prop.substring(0, prop.length-16);
      const [foundPkgName, foundFunc] = Object
        .entries(imports.wbg)
        .find(([name, _f]) => name.includes(pkgName));
      if (foundPkgName) {
        target[foundPkgName] = foundFunc;
        return foundFunc;
      }
      return Reflect.get(...arguments);
    },
  });

}

run();
