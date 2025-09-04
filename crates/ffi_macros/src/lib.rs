#![crate_type = "proc-macro"]

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Field, Fields, ItemEnum, ItemFn, ItemStruct, Type, TypePath, parse_macro_input};

#[proc_macro_attribute]
pub fn ffi_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let body = &input.block;

    let output = quote! {
        #(#attrs)*
        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        #vis #sig #body
    };

    output.into()
}

#[proc_macro_attribute]
pub fn ffi_record(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    if let Fields::Named(ref mut fields) = input.fields {
        for field in &mut fields.named {
            if is_option_type(field) {
                add_option_field_attributes(field);
            }
        }
    }

    let struct_attrs = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        #[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
    };

    // Combine original attributes with new ones
    let combined = quote! {
        #struct_attrs
        #input
    };

    combined.into()
}

#[proc_macro_attribute]
pub fn ffi_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);

    let enum_attrs = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        #[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Enum))]
    };

    // Combine original attributes with new ones
    let combined = quote! {
        #enum_attrs
        #input
    };

    combined.into()
}

fn is_option_type(field: &Field) -> bool {
    if let Type::Path(TypePath { path, .. }) = &field.ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn add_option_field_attributes(field: &mut Field) {
    let field_name = field.ident.to_token_stream().to_string();

    if field_name != "genesis_id" && field_name != "genesis_hash" {
        let uniffi_attr: syn::Attribute =
            syn::parse_quote!(#[cfg_attr(feature = "ffi_uniffi", uniffi(default = None))]);
        field.attrs.push(uniffi_attr);
    }
}
