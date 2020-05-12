extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use syn::parse::{ParseStream, Parse};
use syn::token::Token;
use syn::export::Span;

#[proc_macro_attribute]
pub fn static_page(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(input as syn::ItemFn);

    let attrs = parse_macro_input!(args as AttributeArgs);

    let path = match &attrs[0] {
        NestedMeta::Lit(syn::Lit::Str(lit)) => Ok(lit.value()),
        _ => Err("No path"),
    }.unwrap();

    let file = match &attrs[1] {
        NestedMeta::Lit(syn::Lit::Str(lit)) => Ok(lit.value()),
        _ => Err("No file"),
    }.unwrap();

    let i = &func.sig.ident;

    let inside_ident = Ident::new(&format!("new_{}", i), Span::call_site());

    let gen = quote! {
        fn #inside_ident(prefix: &str) -> String {
            use std::fs;
            let content = fs::read_to_string(format!("{}/{}", prefix, #file))
                .unwrap_or_else(|error| { error.to_string() });
            content
        }

        fn #i() -> Route {
            Route {
                path: #path,
                render_action: #inside_ident,
            }
        }
    };

    gen.into()
}