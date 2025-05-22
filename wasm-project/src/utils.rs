use std::cell::UnsafeCell;

use js_sys::{ArrayBuffer, Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
    #[wasm_bindgen(js_namespace = window)]
    fn __get_hmr_import_obj() -> Object;

    #[wasm_bindgen(js_namespace = window)]
    fn __get_wasm_module_instance(path: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    async fn __get_module_instance_test() -> JsValue;
}

pub async fn get_wasm(mod_path: &str) -> Result<WebAssembly::Instance, JsValue> {
    // log("marker 1");
    // let t = __get_module_instance_test().await;
    // log(&t.js_typeof().as_string().unwrap());
    // let t =  __get_wasm_module_instance(mod_path.into()).await;
    // log(&t.js_typeof().as_string().unwrap());
    // log("marker 2");

    // let instance = __get_wasm_module_instance(mod_path.into());
    //
    // log("Hello from get_wasm!!");
    // let instance: WebAssembly::Instance = instance
    //     .dyn_into()?;

    let resp = web_sys::window().unwrap_throw().fetch_with_str(mod_path);
    let wasm = JsFuture::from(WebAssembly::instantiate_streaming(
        &resp,
        &__get_hmr_import_obj(),
    ))
    .await?;
    let instance: WebAssembly::Instance = Reflect::get(&wasm, &"instance".into())?.dyn_into()?;
    Ok(instance)
}

// pub struct WasmBinInfo {
//     pub bytes: Vec<u8>,
// }
//
// pub async fn get_wasm_bin(mod_path: &str) -> Result<WasmBinInfo, JsValue> {
//     let resp = web_sys::window().unwrap_throw().fetch_with_str(mod_path);
//     let wasm = JsFuture::from(resp).await?;
//     let array_buffer: ArrayBuffer = wasm.dyn_into()?;
//     let bytes = js_sys::Uint8Array::new(&array_buffer).to_vec();
//     Ok(WasmBinInfo { bytes })
// }

// pub static UNKO: u32 = 56;

thread_local! {
    pub static TEST_OBJECT: Test = Test {
        a: 34,
        b: 78,
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Test {
    pub a: i32,
    pub b: i32,
}

pub fn with_test_object<F: FnOnce(&Test) -> R, R>(f: F) -> R {
    TEST_OBJECT.with(|test| f(test))
}
