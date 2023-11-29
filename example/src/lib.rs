use leptos::*;
use wasm_bindgen::prelude::*;
mod comp1;
mod comp2;
use comp1::*;
use comp2::*;
// note:
// Comp: Box<dyn leptos::Component<View>>
//
fn update_comp1(set_signal: WriteSignal<View>, Comp: Box<dyn IntoView>) {
    set_signal(Comp.into_view());
}
#[component]
pub fn App() -> impl IntoView {
    let mut version: u8 = 0;
    let (comp1, set_comp1) = create_signal(view! { <Comp1 /> });
    let switch_comp = move |_| {
        version += 1;
        set_comp1(view! { <Comp2 /> });
    };
    type V = Box<dyn IntoView>;
    
    view! {
        <div>
            {comp1}
            <button on:click=switch_comp> "Switch" </button>
        </div>
    }
}

#[wasm_bindgen]
pub fn main() {
    leptos::mount_to_body(|| view! { <div> <App /> </div> })
}

