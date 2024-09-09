use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType, Signature};

#[proc_macro_attribute]
pub fn sync(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let ItemFn { attrs, vis, sig, block } = input_fn;
    let Signature { ident, generics, inputs, output, .. } = sig;

    let return_type = match output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    let gen = quote! {
        #(#attrs)*
        #vis fn #ident #generics(#inputs) -> #return_type {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                #block
            })
        }
    };

    gen.into()
}

#[cfg(test)]
mod tests {
    use trybuild::TestCases;

    #[test]
    fn test_tokio_sync_wrapper() {
        let t = TestCases::new();
        t.pass("tests/01-basic-usage.rs");
        t.pass("tests/02-with-arguments.rs");
        t.pass("tests/03-different-return-types.rs");
        t.pass("tests/04-public-function.rs");
    }
}
