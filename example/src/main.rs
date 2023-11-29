use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() {
    leptos::mount_to_body(|| view! { <div> "Hello World!" </div> })
}
