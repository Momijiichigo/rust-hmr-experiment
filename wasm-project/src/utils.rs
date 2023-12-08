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
}
impl ImportObject {
    fn new() -> Result<Self, JsValue> {
        Ok(Self {
            __wbindgen_placeholder__: Reflect::get(&__wbg_get_imports(), &"wbg".into())?.into(),
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
