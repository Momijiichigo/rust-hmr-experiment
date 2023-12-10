use js_sys::{ArrayBuffer, Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
    #[wasm_bindgen(js_namespace = window)]
    fn __wbg_get_imports() -> Object;
}

#[wasm_bindgen]
struct ImportObject {
    __wbindgen_placeholder__: Object,
    __wbindgen_externref_xform__: ExternrefXForm
}
struct ExternrefXForm {
    __wbindgen_externref_table_grow: Closure<dyn Fn(u32)->u32>,
    __wbindgen_externref_table_set_null: Closure<dyn Fn(u32)>,
}

impl ImportObject {
    fn new() -> Result<Self, JsValue> {
        Ok(Self {
            __wbindgen_placeholder__: Reflect::get(&__wbg_get_imports(), &"wbg".into())?.into(),
            __wbindgen_externref_xform__: (ExternrefXForm {
                __wbindgen_externref_table_grow: Closure::new(|delta: u32| -> u32 {
                    let table: WebAssembly::Table = wasm_bindgen::function_table().into();
                    table.grow(delta).unwrap_throw()
                }),
                __wbindgen_externref_table_set_null: Closure::new(|index: u32| {
                    let table: WebAssembly::Table = wasm_bindgen::function_table().into();
                    table.set(index, &Function::default()).unwrap_throw();
                }),
            }).into(),
        })
    }
}
pub async fn get_wasm(mod_path: &str) -> Result<WebAssembly::Instance, JsValue> {
    let resp = web_sys::window().unwrap_throw().fetch_with_str(mod_path);
    let wasm = JsFuture::from(WebAssembly::instantiate_streaming(
        &resp,
        &JsValue::from(ImportObject::new()?).into(),
    ))
    .await.unwrap_throw();
    let instance: WebAssembly::Instance = Reflect::get(&wasm, &"instance".into())?.dyn_into()?;
    Ok(instance)
}
