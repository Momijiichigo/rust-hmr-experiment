use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[wasm_bindgen]
extern "C" {
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
    let resp = JsFuture::from(web_sys::window().unwrap_throw().fetch_with_str(mod_path)).await?;
    let resp: web_sys::Response = resp.dyn_into()?;
    let buffer: js_sys::Uint8Array = JsFuture::from(resp.array_buffer()?).await?.dyn_into()?;
    let buffer: Vec<u8> = buffer.to_vec();
    let wasm = JsFuture::from(WebAssembly::instantiate_buffer(
        &buffer,
        &JsValue::from(ImportObject::new()?).into(),
        
    ))
    .await?;
    let instance: WebAssembly::Instance = Reflect::get(&wasm, &"instance".into())?.dyn_into()?;
    Ok(instance)
}
