use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated, Expr, ItemFn, PatType, Result, ReturnType, Signature, Stmt, Token, Type};

struct ClosureArg<P: Parse> {
    pat: P,
    colon_token: Option<Token![:]>,
    ty: Option<Type>,
}

struct ClosureInput<P: Parse> {
    args: Punctuated<ClosureArg<P>, Token![,]>,
    body: Expr,
}

struct BlockInput {
    stmts: Vec<Stmt>,
    expr: Option<Expr>,
}

impl<P: Parse> Parse for ClosureArg<P> {
    fn parse(input: ParseStream) -> Result<Self> {
        let pat: P = input.parse()?;
        let (colon_token, ty) = if input.peek(Token![:]) {
            let colon_token = input.parse()?;
            let ty = input.parse()?;
            (Some(colon_token), Some(ty))
        } else {
            (None, None)
        };
        Ok(ClosureArg { pat, colon_token, ty })
    }
}

impl<P: Parse> Parse for ClosureInput<P> {
    fn parse(input: ParseStream) -> Result<Self> {
        let args = if input.peek(Token![|]) {
            let _: Token![|] = input.parse()?;
            let mut args = Punctuated::new();
            while !input.peek(Token![|]) {
                let arg: ClosureArg<P> = input.parse()?;
                args.push_value(arg);
                if input.peek(Token![|]) {
                    break;
                }
                let punct: Token![,] = input.parse()?;
                args.push_punct(punct);
            }
            let _: Token![|] = input.parse()?;
            args
        } else if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            content.parse_terminated(ClosureArg::parse, Token![,])?
        } else {
            return Err(input.error("expected closure arguments"));
        };

        input.parse::<Token![=>]>()?;
        let body = input.parse()?;

        Ok(ClosureInput { args, body })
    }
}

impl Parse for BlockInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut stmts = Vec::new();
        let mut expr = None;

        while !input.is_empty() {
            if input.fork().parse::<Stmt>().is_ok() {
                stmts.push(input.parse()?);
            } else {
                expr = Some(input.parse()?);
                break;
            }
        }

        Ok(BlockInput { stmts, expr })
    }
}

#[proc_macro]
pub fn closure(input: TokenStream) -> TokenStream {
    let input: ClosureInput<PatType> = parse_macro_input!(input);
    let ClosureInput { args, body } = input;

    let args = args.iter().map(|arg| {
        let ClosureArg { pat, colon_token, ty } = arg;
        quote! { #pat #colon_token #ty }
    });

    let gen = quote! {{
        |#(#args),*| {
            let fut = async move { #body };
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(fut)
        }
    }};

    gen.into()
}

#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    let input: BlockInput = parse_macro_input!(input);
    let BlockInput { stmts, expr } = input;

    let gen = if let Some(final_expr) = expr {
        quote! {{
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                #(#stmts)*
                #final_expr
            })
        }}
    } else {
        quote! {{
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                #(#stmts)*
            })
        }}
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn sync(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block } = parse_macro_input!(item);
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
        t.pass("tests/*.rs");
    }
}
