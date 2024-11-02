#[allow(unused_extern_crates)]
extern crate proc_macro;

use quote::ToTokens;
use syn::{parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn runtime(_args: proc_macro::TokenStream, input_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input_stream
    // let input_stream: proc_macro2::TokenStream = input_stream.into();
    // let mut input: ItemFn = match syn::parse2(input_stream.clone()) {
    //     Ok(it) => it,
    //     Err(_) => panic!("The `runtime` macro attribute is only valid when called on a fn."),
    // };
    // input.sig.asyncness = None;
    // let block = input.block;
    // input.block = parse_quote! {
    //     shared_tokio_runtime::rt().block_on(async {
    //         #block
    //     })
    // };
    // input.into_token_stream().into()
}