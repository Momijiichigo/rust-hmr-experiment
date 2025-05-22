use js_sys::{Function, Object, Reflect, WebAssembly};
use leptos::html::{div, HtmlElement};
use leptos::tachys::dom::body;
use leptos::{prelude::View, *};
use leptos::prelude::{ElementChild, Mountable};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// #[no_mangle]
#[unsafe(export_name = "func_a")]
pub fn func_a() {
    log!("Hello from mod1.wasm!!");
}


#[unsafe(export_name = "func_b")]
pub fn func_b(input: i32) -> i32 {
    2 * input
}



#[component]
pub fn ComponentA() -> impl IntoView {
    view! {
        <div>
            "Hello from ComponentA!"
        </div>
    }
}

use leptos::mount::{self, mount_to, mount_to_body};

#[unsafe(export_name = "mount_component_a")]
pub fn ComponentA_mount_to(parent: web_sys::HtmlElement) {
    let unmount_handle = mount_to(parent, ComponentA);
    unmount_handle.forget();
}
// #[wasm_bindgen]
// pub struct _View(View<()>);

// #[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
// pub fn __Comp1() -> impl IntoView {
//     view! {
//         <div>
//             "Hello from ComponentA!"
//         </div>
//     }
// }


#[cfg(not(feature = "separate-comp"))]
use crate::utils::{Test, with_test_object};
#[cfg(feature = "separate-comp")]
use wasm_project::utils::{Test, with_test_object};
#[unsafe(export_name = "access_thread_local_static")]
pub fn access_thread_local_static() {
    log!("- Before exec....");
    log!("- test object: {:?}", with_test_object(|test| {
        test.clone()
    }));
    log!("- After exec");
}

#[unsafe(export_name = "passing_reference")]
pub fn passing_reference(input: &str) -> i32 {
    log!("Passing reference test: {:?}", input);
    45
}

