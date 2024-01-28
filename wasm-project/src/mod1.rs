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
    log!("Hello from ComponentA!");
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
pub fn test_comp() -> impl IntoView {
    leptos_dom::Component::new("Test", move || __Comp1())
}
#[wasm_bindgen]
pub fn ComponentA_into_view() -> _View {
    log!("Before exec...");
    // let val = test_comp().into_view();
    // let val = __Comp1().into_view();

    // let val = leptos_reactive::untrack_with_diagnostics(|| {
    //     __Comp1().into_view()
    // });
    let val = leptos_reactive::untrack(|| {
        log!("In untrack");
        // __Comp1().into_view()
    });
    
    log!("{:?}", val);
    log!("After exec");
    _View {
        view: ComponentA().into_view(),
    }
}

pub fn test1<T>(f: impl FnOnce() -> T) -> T {
    let result = f();

    result
}
#[wasm_bindgen]
pub struct _View {
    view: View,
}
