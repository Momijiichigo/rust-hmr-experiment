
import init, {get_wasm_table} from './pkg/wasm_project.js';

const hmr_import_obj = {}
window.__get_hmr_import_obj = () => hmr_import_obj;

window.__get_wasm_module_instance = async (path) => {
  // const res = await fetch(path).then(response => response.arrayBuffer());
  // const module = await WebAssembly.instantiate(res, hmr_import_obj);
  // return module.instance;

  const res = await WebAssembly.instantiateStreaming(fetch(path), __get_hmr_import_obj())
  console.log('res.instance', res.instance);
  return res.instance;
}

window.__get_module_instance_test = () => {
  return new Promise(resolve=>setTimeout(()=>resolve(hmr_import_obj), 1000));
}
async function run() {
  // keep these public for debugging
  window.project = await init();
  window.wasmTable = get_wasm_table();
  window.imports = __wbg_get_imports();

  // hmr_import_obj.__wbindgen_externref_xform__ = {
  //   __wbindgen_externref_table_grow: (delta) => wasmTable.grow(delta),
  //   __wbindgen_externref_table_set_null: (idx) => wasmTable.set(idx),
  // }
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
      if (prop === "__stack_pointer") {
        return new WebAssembly.Global({value: "i32", mutable: true}, project.__stack_pointer.value);
      }
      if (prop in project) {
        return project[prop];
      }
      // // TODO: temporary fix, need review
      // if (prop.indexOf("core::panicking::panic_nounwind") === 0) {
      //   return project['rust_panic']
      // }
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

      // some functions with name including "usize" are not generated
      // substitute it with "u32" or "i32"
      if (prop.includes("usize")) {
        return find32SubstitutedName(project, prop, "usize");
      }
      if (prop.includes("i32")) {
        return find32SubstitutedName(project, prop, "i32");
      }
      if (prop.includes("u32")) {
        return find32SubstitutedName(project, prop, "u32");
      }

    }
  }
  );

  hmr_import_obj.__wbindgen_placeholder__ = new Proxy(imports.wbg, {
    get(target, prop, _receiver) {
      if (prop in target) {
        return Reflect.get(...arguments);
      }
      if (prop === "__wbindgen_describe") {
        return () => {};
      }
      const pkgName = prop.substring(0, prop.length - 16);
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

/**
 * Find the function with the same name but with different 32bit type names in it
 *
 * i.e. `u32`, `i32`, or `usize`
 *
 * @param {Object} project
 * @param {string} prop
 * @param {"u32" | "i32" | "usize"} type32
 *
 */
function find32SubstitutedName(project, prop, type32) {
  // strip the last characters after "::"
  const funcName = prop.substring(0, prop.length - 19)

  const possibleSubstitutes = [];
  let found32Type = null;
  if (type32 === "usize") {
    found32Type = "usize";
    possibleSubstitutes.push("u32", "i32");
  } else if (type32 === "i32") {
    found32Type = "i32";
    possibleSubstitutes.push("usize", "u32");
  } else if (type32 === "u32") {
    found32Type = "u32";
    possibleSubstitutes.push("usize", "i32");
  }
  const replacedNames = possibleSubstitutes.map(substitute => funcName.replace(found32Type, substitute));


  for (const name of replacedNames) {
    const [foundFuncName, func] = Object
      .entries(project)
      .find(([key, _value]) => key.indexOf(name) === 0) || [undefined, undefined];
    if (foundFuncName) {
      return func;
    }
  }

  console.error(`Cannot find function for ${funcName}`);


}
run();
