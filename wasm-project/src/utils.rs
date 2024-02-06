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
    let instance: WebAssembly::Instance = Reflect::get(&wasm, &"instance".into())?.dyn_into()?;
    Ok(instance)
}

// thread_local! {
//     pub static TEST_OBJECT: Test = Test {
//         a: 1,
//         b: 2,
//     };
// }
pub const TEST_OBJECT: ::std::thread::LocalKey<Test> = {
    #[inline]
    fn __init() -> Test {
        Test { a: 50, b: 165 }
    }
    #[inline]
    unsafe fn __getit(
        init: ::std::option::Option<&mut ::std::option::Option<Test>>,
    ) -> ::std::option::Option<&'static Test> {
        #[thread_local]
        static __KEY: ::std::thread::local_impl::Key<Test> =
            ::std::thread::local_impl::Key::<Test>::new();
        unsafe {
            __KEY.get(move || {
                // if let ::std::option::Option::Some(init) = init {
                //     if let ::std::option::Option::Some(value) = init.take() {
                //         return value;
                //     } else if true {
                //         {
                //             ::core::panicking::panic_fmt(format_args!(
                //                 "internal error: entered unreachable code: {0}",
                //                 format_args!("missing default value"),
                //             ));
                //         };
                //     }
                // }
                __init()
            })
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
#[derive(Debug, Clone, Copy)]
pub struct Test {
    pub a: i32,
    pub b: i32,
}

pub fn with_test_object<F: FnOnce(&Test) -> R, R>(f: F) -> R {

    TEST_OBJECT.with(|test| f(test))
}
