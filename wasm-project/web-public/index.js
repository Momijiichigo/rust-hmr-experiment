
import init, { add, get_wasm_table, get_wasm_memory, get_imports } from './pkg/wasm_project.js';


const hmr_import_obj = {}
window.__hmr_import_obj = hmr_import_obj;


async function run() {
  await init();
  window.wasmTable = get_wasm_table();
  window.wasmMemory = get_wasm_memory();
  window.imports = get_imports();
  hmr_import_obj.__wbindgen_placeholder__ = imports.wbg;
  hmr_import_obj.__wbindgen_externref_xform__ = {
    __wbindgen_externref_table_grow: (delta) => wasmTable.grow(delta),
    __wbindgen_externref_table_set_null: (idx) => wasmTable.set(idx),
  }

  hmr_import_obj.__wbindgen_placeholder__ = new Proxy(imports.wbg, {
    get(target, prop, receiver) {
      if (prop in target) {
        return Reflect.get(...arguments);
      }
      if (prop === "__wbindgen_describe") {
	      return () => {};
      }
      const pkgName = prop.substring(0, prop.length-16);
      const [foundPkgName, foundFunc] = Object
        .entries(imports.wbg)
        .find(([name, f]) => name.includes(pkgName));
      if (foundPkgName) {
        target[foundPkgName] = foundFunc;
        return foundFunc;
      }
      return Reflect.get(...arguments);
    },
  });

  console.log(add(1, 2));
}

run();
