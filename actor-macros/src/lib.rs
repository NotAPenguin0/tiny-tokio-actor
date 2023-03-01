extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::parse::Parse;
use proc_macro::{TokenStream};

fn impl_message(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let response =
        ast.attrs.first().
            map(|a|
                a.parse_args::<syn::Type>().unwrap()
            )
            .unwrap_or(
                syn::parse_str("()").unwrap()
            );
    let stream = quote! {
        impl tiny_tokio_actor::Message for #name {
            type Response = #response;
        }
    };
    stream.into()
}

#[proc_macro_derive(Message, attributes(response))]
pub fn derive_message_impl(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    let gen = impl_message(&ast);
    gen
}