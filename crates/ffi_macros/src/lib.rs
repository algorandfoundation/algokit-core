use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn ffi_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let js_name = &input.sig.ident.to_string().to_case(Case::Camel);
    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let body = &input.block;

    let output = quote! {
        #(#attrs)*
        #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = #js_name))]
        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        #vis #sig #body
    };

    output.into()
}
