use std::cell::UnsafeCell;

use js_sys::{ArrayBuffer, Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
    #[wasm_bindgen(js_namespace = window)]
    fn __get_hmr_import_obj() -> Object;
}

pub async fn get_wasm(mod_path: &str) -> Result<WebAssembly::Instance, JsValue> {
    let resp = web_sys::window().unwrap_throw().fetch_with_str(mod_path);
    let wasm = JsFuture::from(WebAssembly::instantiate_streaming(
        &resp,
        &__get_hmr_import_obj(),
    ))
    .await?;
    // log("Hello");
    let instance: WebAssembly::Instance = Reflect::get(&wasm, &"instance".into())?.dyn_into()?;
    Ok(instance)
}

// thread_local! {
//     pub static TEST_OBJECT: Test = Test {
//         a: 1,
//         b: 2,
//     };
// }
// use crate::thread;

pub const TEST_OBJECT: ::std::thread::LocalKey<Test> = {
    #[inline]
    fn __init() -> Test {
        Test { a: 1, b: 2 }
    }
    unsafe {
        use ::std::thread::local_impl::LazyStorage;
        use ::std::thread::LocalKey;
        LocalKey::new(|init| {
            static VAL: LazyStorage<Test> = LazyStorage::new();
            VAL.get(init, __init)
            // let cell = UnsafeCell::new(None);

            // let value = &*cell.get();
            // match value {
            //     Some(v) => v,
            //     None => {
            //         let value = init
            //             .and_then(Option::take)
            //             .unwrap_or_else(__init);
            //         // Destroy the old value, after updating the TLS variable as the
            //         // destructor might reference it.
            //         // FIXME(#110897): maybe panic on recursive initialization.
            //         unsafe {
            //             cell.get().replace(Some(value));
            //         }
            //         // SAFETY: we just set this to `Some`.
            //         unsafe { (*cell.get()).as_ref().unwrap_unchecked() }
            //     }
            // }
        })
    }
};

#[derive(Debug, Clone, Copy)]
pub struct Test {
    pub a: i32,
    pub b: i32,
}

pub fn with_test_object<F: FnOnce(&Test) -> R, R>(f: F) -> R {
    TEST_OBJECT.with(|test| f(test))
}
