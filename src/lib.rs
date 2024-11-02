use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn runtime(_args: TokenStream, input_stream: TokenStream) -> TokenStream {
    let mut input: ItemFn = match syn::parse2(input_stream.clone()) {
        Ok(it) => it,
        Err(_) => panic!("The `runtime` macro attribute is only valid when called on a fn."),
    };
    input.sig.asyncness = None;
    let block = input.block;
    input.block = parse_quote! {
        shared_tokio_runtime::rt().block_on(async {
            #block
        })
    };
    input.into_token_stream()
}