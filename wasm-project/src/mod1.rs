use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use leptos::*;

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
    {
        {
            let _ = ::leptos::leptos_dom::html::div;
            ::leptos::leptos_dom::html::div()
        }
    }
}
#[wasm_bindgen]
pub fn ComponentA_into_view() -> _View {
    log!("Before exec");
    let _ = leptos_reactive::untrack_with_diagnostics(|| {
        ComponentA()
    });
    log!("After exec");
    _View {
        view: ComponentA().into_view(),
    }
}

#[wasm_bindgen]
pub struct _View {
    view: View,
}
