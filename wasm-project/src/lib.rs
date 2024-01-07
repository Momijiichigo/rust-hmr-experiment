use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use std::*;
mod mod1;
mod utils;
use utils::get_wasm;
use std::sync::Mutex;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
    // #[wasm_bindgen(js_namespace = window)]
    // fn __wbg_get_imports() -> JsValue;
    #[wasm_bindgen(js_namespace = window)]
    fn __get_hmr_import_obj() -> Object;
}

macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[wasm_bindgen(start)]
fn run() {
    spawn_local(async {
        main().await.unwrap_throw();
    });
}

async fn main() -> Result<(), JsValue> {
    log!("Hello from main wasm!!");

    // mod1::component_a();

    let instance = get_wasm("wasm/mod1.wasm").await?;
    log!("Got wasm instance");
    let exports = instance.exports();
    let component_a: Function =
        Reflect::get(exports.as_ref(), &"component_a".into())?.dyn_into()?;
    component_a.call0(&JsValue::null())?;
    Ok(())
}


#[wasm_bindgen]
pub fn get_wasm_table() -> Object {
    wasm_bindgen::function_table().into()
}

#[wasm_bindgen]
pub fn get_wasm_memory() -> Object {
    wasm_bindgen::memory().into()
}

