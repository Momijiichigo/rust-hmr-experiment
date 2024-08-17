#![feature(async_closure)]
#![feature(libstd_sys_internals)]
#![feature(rt)]
// needed for debugging thread_local
#![feature(thread_local)]
#![feature(thread_local_internals)]
#![feature(never_type)]
// #![feature(core_panic)]

mod mod1;
pub mod utils;

use js_sys::{Function, Object, Reflect, WebAssembly};
use leptos::*;
use std::*;
use utils::get_wasm;
use wasm_bindgen::{
    convert::{FromWasmAbi, TryFromJsValue, WasmSlice},
    prelude::*,
};
use wasm_bindgen_futures::{spawn_local, JsFuture};

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
    let (elem, set_elem) = create_signal(view! {
        <div>
            <ComponentA />
        </div>
    });
    // let task = create_local_resource(|| (), move |_| async {
    //     let instance = get_wasm("wasm/mod1.wasm").await?;
    //     log!("Got wasm instance");
    //     let exports = instance.exports();
    //     let component_a: Function =
    //         Reflect::get(exports.as_ref(), &"ComponentA".into())?.dyn_into()?;
    //     let component_a: _View = _View::try_from_js_value(component_a.call0(&JsValue::null())?)?;
    //     let component_a: View = unsafe {
    //         std::mem::transmute(component_a)
    //     };
    //     // unsafe {
    //     //     let raw_component_address: u32 = std::mem::transmute(
    //     //         component_a
    //     //             .clone()
    //     //             .call0(&JsValue::null())
    //     //             .expect_throw("failed to construct component a"),
    //     //     );
    //     // };
    //     // let component_a_constructor = || unsafe {
    //     //     // pointer address to the the return value of ComponentA()
    //     //     // which is a struct implements IntiView
    //     //     let raw_component_address: u32 = std::mem::transmute(
    //     //         component_a
    //     //             .clone()
    //     //             .call0(&JsValue::null())
    //     //             .expect_throw("failed to construct component a"),
    //     //     );
    //     //     // let component: *mut dyn IntoView = FromWasmAbi::from_abi(WasmSlice {
    //     //     //     ptr: raw_component_address,
    //     //     //     len: 1,
    //     //     // });
    //     // };
    //     elem().child(component_a.into_view());
    //     // component_a.call0(&JsValue::null())?;
    //     Ok::<(), JsValue>(())
    // });
    elem
}

async fn main() -> Result<(), JsValue> {
    log!("Hello from main wasm!!");

    let instance = get_wasm("wasm/mod1.wasm").await?;
    log!("Got wasm instance");
    let exports = instance.exports();
    let func_a: Function = Reflect::get(exports.as_ref(), &"func_a".into())?.dyn_into()?;
    log!("# marker 0");

    func_a.call0(&JsValue::null())?;

    log!("# First invokation of investigate_problem!");
    mod1::investigate_problem();

    log!("# marker A");
    let investigate_problem: Function =
        Reflect::get(exports.as_ref(), &"investigate_problem".into())?.dyn_into()?;
    log!("# Second invokation of investigate_problem!");
    investigate_problem.call0(&JsValue::null())?;

    log!("# marker B");
    let component_a: Function =
        Reflect::get(exports.as_ref(), &"ComponentA_into_view".into())?.dyn_into()?;
    log!("# marker C");
    let component_a: _View = _View::try_from_js_value(component_a.call0(&JsValue::null())?)?;
    // unsafe {

    //     log!("# First invokation of investigate_problem2!");

    //     let input = "Hello from wasm!";
    //     mod1::investigate_problem2(input);

    //     log!("# marker A");
    //     let investigate_problem: Function =
    //         Reflect::get(exports.as_ref(), &"investigate_problem2".into())?.dyn_into()?;
    //     log!("# Second invokation of investigate_problem!");
    //     let (input0, input1) = std::mem::transmute::<&str, (usize, usize)>(input);
    //     investigate_problem.call2(&JsValue::null(), &JsValue::from(input0), &JsValue::from(input1))?;
    // }

    // unsafe {
    //     let raw_component_address: u32 = std::mem::transmute(
    //         component_a
    //             .call0(&JsValue::null())
    //             .expect_throw("failed to construct component a"),
    //     );
    //     log!("raw_component_address: {}", raw_component_address);
    // };
    log!("# marker D");
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
