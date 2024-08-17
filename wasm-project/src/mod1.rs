use js_sys::{Function, Object, Reflect, WebAssembly};
use leptos::*;
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

#[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
pub fn __Comp1() -> impl IntoView {
    view! {
        <div>
            "Hello from ComponentA!"
        </div>
    }
}

#[cfg(not(feature = "separate-comp"))]
use crate::utils::{Test, with_test_object};
#[cfg(feature = "separate-comp")]
use wasm_project::utils::{Test, with_test_object};
#[wasm_bindgen]
pub fn investigate_problem() {
    log!("- Before exec....");
    // let val = test_comp().into_view();
    // let val = __Comp1().into_view();

    // let val = leptos_reactive::untrack_with_diagnostics(|| {
    //     __Comp1().into_view()
    // });
    // log!("## get test object");
    // let mut test_addr: *const Test = std::ptr::null();
    // with_test_object(|test| unsafe {
    //     let test = test.clone();
    //     test_addr = std::mem::transmute(&test)
    // });
    // let test = with_test_object(|test| test);
    // log!("test object: {:?}", test_addr);
    log!("- test object: {:?}", with_test_object(|test| {
        let test_addr = test as *const Test;
        test_addr
    }));
    // let val = leptos_reactive::untrack(|| {
    //     log!("In untrack");
    //     // __Comp1().into_view()
    // });
    
    // log!("{:?}", val);
    log!("- After exec");
}


#[wasm_bindgen]
pub fn investigate_problem2(input: &str) {
    log!("- Before exec....");
    log!("- input: {:?}", input);
    log!("- After exec");
}

#[wasm_bindgen]
pub fn ComponentA_into_view() -> _View {
    _View {
        view: ComponentA().into_view(),
    }
}
#[wasm_bindgen]
pub struct _View {
    view: View,
}
