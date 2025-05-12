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

#[no_mangle]
#[wasm_bindgen]
pub fn func_a() {
    log!("Hello from mod1.wasm!!");
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
#[wasm_bindgen]
pub fn ComponentA_mount_to() {
    // let unmount_handle = mount_to_body(ComponentA);
    // unmount_handle.forget();
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
#[wasm_bindgen]
pub fn access_thread_local_static() {
    log!("- Before exec....");
    log!("- test object: {:?}", with_test_object(|test| {
        test.clone()
    }));
    log!("- After exec");
}


#[wasm_bindgen]
pub fn passing_reference(input: &str) {
    log!("Passing reference test: {:?}", input);
}

