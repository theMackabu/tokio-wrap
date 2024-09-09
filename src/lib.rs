use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn sync(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_generics = &input_fn.sig.generics;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_body = &input_fn.block;

    let return_type = match &input_fn.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    let gen = quote! {
        fn #fn_name #fn_generics(#fn_inputs) -> #return_type {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                #fn_body
            })
        }
    };

    gen.into()
}
