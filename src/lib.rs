extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Item};

#[proc_macro_attribute]
pub fn runtime(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: Item = match parse_macro_input!(input as Item) {
        Item::Fn(mut fn_item) => {
            let block = fn_item.block;
            fn_item.sig.asyncness = None;
            fn_item.block = parse_quote! {
                shared_tokio_runtime::rt().block_on(async {
                    #block
                })
            };
            Item::Fn(fn_item)
        },
        _ => {
            panic!("The `runtime` macro attribute is only valid when called on a fn.")
        }
    };
    TokenStream::from(quote! (#input))
}