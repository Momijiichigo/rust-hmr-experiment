mod mod1 {
    use js_sys::{Function, Object, Reflect, WebAssembly};
    use leptos::html::{div, HtmlElement};
    use leptos::tachys::dom::body;
    use leptos::{prelude::View, *};
    use leptos::prelude::{ElementChild, Mountable};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::{spawn_local, JsFuture};
    #[automatically_derived]
    const _: () = {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __wbindgen_describe___wbg_log_ec64a1a3d767c123() {
            use wasm_bindgen::describe::*;
            wasm_bindgen::__rt::link_mem_intrinsics();
            inform(FUNCTION);
            inform(0);
            inform(1u32);
            <&str as WasmDescribe>::describe();
            <() as WasmDescribe>::describe();
            <() as WasmDescribe>::describe();
        }
    };
    #[allow(nonstandard_style)]
    #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
    fn log(a: &str) {
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        extern "C" {
            fn __wbg_log_ec64a1a3d767c123(
                a_1: <<&str as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                a_2: <<&str as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                a_3: <<&str as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                a_4: <<&str as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> ();
        }
        unsafe {
            let _ret = {
                let a = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a);
                let (a_1, a_2, a_3, a_4) = <<&str as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::split(
                    a,
                );
                __wbg_log_ec64a1a3d767c123(a_1, a_2, a_3, a_4)
            };
            ()
        }
    }
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::__rt::{flat_len, flat_byte_slices};
        static _INCLUDED_FILES: &[&str] = &[];
        const _ENCODED_BYTES: &[u8] = {
            const _CHUNK_SLICES: [&[u8]; 1usize] = [
                b"\0\0\x01\0\x01\x01\x07console\0\x1a__wbg_log_ec64a1a3d767c123\0\0\0\0\x01\x01\x01a\0\0\0\x03log\x01\x01\0\0\0\0\0\0\0\x1dwasm-project-04fe3f9785e81900\0\0",
            ];
            #[allow(long_running_const_eval)]
            const _CHUNK_LEN: usize = flat_len(_CHUNK_SLICES);
            #[allow(long_running_const_eval)]
            const _CHUNKS: [u8; _CHUNK_LEN] = flat_byte_slices(_CHUNK_SLICES);
            const _LEN_BYTES: [u8; 4] = (_CHUNK_LEN as u32).to_le_bytes();
            const _ENCODED_BYTES_LEN: usize = _CHUNK_LEN + 4;
            #[allow(long_running_const_eval)]
            const _ENCODED_BYTES: [u8; _ENCODED_BYTES_LEN] = flat_byte_slices([
                &_LEN_BYTES,
                &_CHUNKS,
            ]);
            &_ENCODED_BYTES
        };
        const _PREFIX_JSON_BYTES: &[u8] = b"0\0\0\0{\"schema_version\":\"0.2.100\",\"version\":\"0.2.100\"}";
        const _ENCODED_BYTES_LEN: usize = _ENCODED_BYTES.len();
        const _PREFIX_JSON_BYTES_LEN: usize = _PREFIX_JSON_BYTES.len();
        const _LEN: usize = _PREFIX_JSON_BYTES_LEN + _ENCODED_BYTES_LEN;
        #[link_section = "__wasm_bindgen_unstable"]
        #[allow(long_running_const_eval)]
        static _GENERATED: [u8; _LEN] = flat_byte_slices([
            _PREFIX_JSON_BYTES,
            _ENCODED_BYTES,
        ]);
    };
    #[unsafe(export_name = "func_a")]
    pub fn func_a() {
        log(&format_args!("Hello from mod1.wasm!!").to_string());
    }
    /// Props for the [`ComponentA`] component.
    ///
    #[builder(crate_module_path = ::leptos::typed_builder)]
    #[allow(non_snake_case)]
    pub struct ComponentAProps {}
    #[automatically_derived]
    impl ComponentAProps {
        /**
                Create a builder for building `ComponentAProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `ComponentAProps`.
                */
        #[allow(dead_code, clippy::default_trait_access)]
        pub fn builder() -> ComponentAPropsBuilder<()> {
            ComponentAPropsBuilder {
                fields: (),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub struct ComponentAPropsBuilder<TypedBuilderFields = ()> {
        fields: TypedBuilderFields,
        phantom: ::core::marker::PhantomData<()>,
    }
    #[automatically_derived]
    impl<TypedBuilderFields> Clone for ComponentAPropsBuilder<TypedBuilderFields>
    where
        TypedBuilderFields: Clone,
    {
        #[allow(clippy::default_trait_access)]
        fn clone(&self) -> Self {
            Self {
                fields: self.fields.clone(),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl ComponentAPropsBuilder<()> {
        #[allow(
            clippy::default_trait_access,
            clippy::used_underscore_binding,
            clippy::no_effect_underscore_binding
        )]
        pub fn build(self) -> ComponentAProps {
            let () = self.fields;
            #[allow(deprecated)] ComponentAProps {}.into()
        }
    }
    #[allow(missing_docs)]
    impl ::leptos::component::Props for ComponentAProps {
        type Builder = ComponentAPropsBuilder;
        fn builder() -> Self::Builder {
            ComponentAProps::builder()
        }
    }
    #[allow(non_snake_case, clippy::too_many_arguments)]
    #[allow(clippy::needless_lifetimes)]
    pub fn ComponentA() -> impl IntoView {
        ::leptos::reactive::graph::untrack_with_diagnostics(move || { __ComponentA() })
    }
    #[doc(hidden)]
    #[allow(
        non_snake_case,
        dead_code,
        clippy::too_many_arguments,
        clippy::needless_lifetimes
    )]
    pub fn __ComponentA() -> impl IntoView {
        {
            #[allow(unused_braces)]
            {
                ::leptos::prelude::View::new(
                        ::leptos::tachys::html::element::div()
                            .child(
                                #[allow(unused_braces)]
                                {
                                    ::leptos::tachys::view::static_types::Static::<
                                        "Hello from ComponentA!",
                                    >
                                },
                            ),
                    )
                    .with_view_marker("wasm-project-src-mod1.rs-27")
            }
        }
    }
    use leptos::mount::{self, mount_to, mount_to_body};
    pub fn ComponentA_mount_to() {}
    use crate::utils::{Test, with_test_object};
    #[unsafe(export_name = "access_thread_local_static")]
    pub fn access_thread_local_static() {
        log(&format_args!("- Before exec....").to_string());
        log(
            &format_args!(
                "- test object: {0:?}",
                with_test_object(|test| { test.clone() }),
            )
                .to_string(),
        );
        log(&format_args!("- After exec").to_string());
    }
    #[unsafe(export_name = "passing_reference")]
    pub fn passing_reference(input: &str) -> i32 {
        log(&format_args!("Passing reference test: {0:?}", input).to_string());
        45
    }
}
