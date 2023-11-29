use proc_macro::TokenStream;


#[proc_macro_attribute]
pub fn my_macro(input: TokenStream, _: TokenStream) -> TokenStream {
    
    input
}

