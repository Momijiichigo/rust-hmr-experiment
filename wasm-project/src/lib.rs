#![feature(async_closure)]
use js_sys::{Function, Object, Reflect, WebAssembly};
use std::*;
use wasm_bindgen::{
    convert::{FromWasmAbi, WasmSlice},
    prelude::*,
};
use wasm_bindgen_futures::{spawn_local, JsFuture};
mod mod1;
mod utils;
use leptos::*;
use utils::get_wasm;

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
    mount_to_body(|| {
        view! {
            <div>
                "Hello from leptos!"
                <App />
            </div>
        }
    })
}
use mod1::*;
#[component]
pub fn App() -> impl IntoView {
    let elem = view! {
        <div>
            <ComponentA />
        </div>
    };
    // let task = async || -> Result<(), JsValue> {
    //     let instance = get_wasm("wasm/mod1.wasm").await?;
    //     log!("Got wasm instance");
    //     let exports = instance.exports();
    //     let component_a: Function =
    //         Reflect::get(exports.as_ref(), &"ComponentA".into())?.dyn_into()?;
    //     unsafe {
    //         let raw_component_address: u32 = std::mem::transmute(
    //             component_a
    //                 .clone()
    //                 .call0(&JsValue::null())
    //                 .expect_throw("failed to construct component a"),
    //         );
    //     };
    //     let component_a_constructor = || unsafe {
    //         // pointer address to the the return value of ComponentA()
    //         // which is a struct implements IntiView
    //         let raw_component_address: u32 = std::mem::transmute(
    //             component_a
    //                 .clone()
    //                 .call0(&JsValue::null())
    //                 .expect_throw("failed to construct component a"),
    //         );
    //         // let component: *mut dyn IntoView = FromWasmAbi::from_abi(WasmSlice {
    //         //     ptr: raw_component_address,
    //         //     len: 1,
    //         // });
    //     };
    //     // elem.child(leptos::component_view(component_a_constructor, ()));
    //     component_a.call0(&JsValue::null())?;
    //     Ok(())
    // };
    // spawn_local(async {
    //     task().await.unwrap_throw();
    // });

    elem
}

async fn main() -> Result<(), JsValue> {
    log!("Hello from main wasm!!");


    let instance = get_wasm("wasm/mod1.wasm").await?;
    log!("Got wasm instance");
    let exports = instance.exports();
    let component_a: Function = Reflect::get(exports.as_ref(), &"func_a".into())?.dyn_into()?;
    log!("marker 0");

    component_a.call0(&JsValue::null())?;

    log!("marker A");
    let component_a: Function = Reflect::get(exports.as_ref(), &"ComponentA_into_view".into())?.dyn_into()?;
    log!("marker B");
    unsafe {
        let raw_component_address: u32 = std::mem::transmute(
            component_a
                .call0(&JsValue::null())
                .expect_throw("failed to construct component a"),
        );
        log!("raw_component_address: {}", raw_component_address);
    };
    log!("marker C");
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
