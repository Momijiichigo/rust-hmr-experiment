
import init, { get_wasm_table } from './pkg/wasm_project.js';

const hmr_import_obj = {}
window.__get_hmr_import_obj = () => hmr_import_obj;

async function run() {
  // keep these public for debugging
  window.project = await init();
  window.wasmTable = get_wasm_table();
  window.imports = __wbg_get_imports();

  hmr_import_obj.__wbindgen_externref_xform__ = {
    __wbindgen_externref_table_grow: (delta) => wasmTable.grow(delta),
    __wbindgen_externref_table_set_null: (idx) => wasmTable.set(idx),
  }
  // ----------- debugging purpose --------------
  hmr_import_obj.env = new Proxy(
    {}, {
      get(_target, prop, _receiver) {
        // if (prop.indexOf("leptos_dom::components::ComponentRepr::new_with_id_concrete") === 0) {
        //   return (...args) => {
        //     const result = project[prop](...args);
        //     console.log("new_with_id_concrete called", args);
        //     return result;
        //   }
        // }
        // if (prop.indexOf("<leptos_dom::components::ComponentRepr as leptos_dom::IntoView>::into_view") === 0) {
        //   return (...args) => {
        //     console.log("ComponentRepr::into_view called", args);
        //     return project[prop](...args);
        //   }
        // }

        if (prop in project) {
          return project[prop];
        }
        // TODO: temporary fix, need review
        if (prop.indexOf("core::panicking::panic_nounwind") === 0) { 
          return project['rust_panic']
        }
        if (prop === "__linear_memory") {
          return project.memory;
        }
	// if (prop === "__stack_pointer") {
	//   // TODO: using wrong value, need fix
	//   return new WebAssembly.Global({value: "i32", mutable: true}, window.heap_next);
	// }
        if (prop === "__indirect_function_table") {
          return wasmTable;
        }

        if (prop.includes("::describe::")) {
          return () => {};
        }
        if (prop.indexOf("wasm_bindgen::__rt::link_mem_intrinsics") === 0) {
          return () => {};
        }

        // some functions with name including "usize" are not generated
        // substitute it with "u32" or "i32"
        if (prop.includes("usize")) {
          // strip the last characters after "::"
          const funcName = prop.substring(0, prop.length-19)
            
          const funcNameU32 = funcName.replace("usize", "u32");
          const funcNameI32 = funcName.replace("usize", "i32");

          const [foundFuncName, func] = (Object
              .entries(project)
              .find(([key, _value]) => key.indexOf(funcNameU32) === 0)
            ) || (Object
              .entries(project)
              .find(([key, _value]) => key.indexOf(funcNameI32) === 0)
            ) || [undefined, undefined];
          if (foundFuncName) {
            return func;
          }
          console.error(`Cannot find function for ${funcName}`);

        }
      }
    }
  );
  // --------------------------------------------
  /*
  // ----------- production purpose --------------
  hmr_import_obj.env = new Proxy(
    project, {
      get(target, prop, _receiver) {
	if (prop in target) {
      	  return Reflect.get(...arguments);
      	}
	if (prop === "__linear_memory") {
	  return project.memory;
	}
	if (prop === "__indirect_function_table") {
	  return wasmTable;
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
  */

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
        .find(([name, _f]) => name.includes(pkgName)) || [undefined, undefined];
      if (foundPkgName) {
        target[foundPkgName] = foundFunc;
        return foundFunc;
      }
      return Reflect.get(...arguments);
    },
  });

}

run();
