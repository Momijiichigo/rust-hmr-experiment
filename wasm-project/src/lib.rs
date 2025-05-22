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

    let instance = get_wasm("wasm/mod1.wasm").await?;
    log!("Got wasm instance");
    let exports = instance.exports();

    // func_a
    {
        let func_a: Function = Reflect::get(exports.as_ref(), &"func_a".into())?.dyn_into()?;

        func_a.call0(&JsValue::null())?;
    }

    // passing_reference

    log!("# Passing reference to mod1.wasm");
    {
        let passing_reference: Function =
            Reflect::get(exports.as_ref(), &"passing_reference".into())?.dyn_into()?;

        let text = "This string's reference is passed from host to mod1.wasm, executed in mod1.wasm using all function dependencies from host.";
        let (addr, len) = unsafe {
            std::mem::transmute::<&str, (usize, usize)>(text)
        };
        // JS number type is f64
        let addr = addr as f64;
        let len = len as f64;
        let r = passing_reference.call2(&JsValue::null(), &addr.into(), &len.into())?;
        log!("Passing reference result: {:?}", r);
    }

    log!("# Calling `access_thread_local_static()` within host.wasm");
    mod1::access_thread_local_static();

    let access_thread_local_static: Function =
        Reflect::get(exports.as_ref(), &"access_thread_local_static".into())?.dyn_into()?;
    log!("# Calling `access_thread_local_static()` in mod1.wasm");
    access_thread_local_static.call0(&JsValue::null())?;

    let mount_component_a: Function =
        Reflect::get(exports.as_ref(), &"mount_component_a".into())?.dyn_into()?;
    log!("# Calling `mount_component_a()` in mod1.wasm");
    mount_component_a.call0(&JsValue::null())?;



    // log!("# marker A");
    // log!("# marker D");

    // let add_component_a: Function =
    //     Reflect::get(exports.as_ref(), &"ComponentA_mount_to".into())?.dyn_into()?;
    // add_component_a.call0(&JsValue::null())?;
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
