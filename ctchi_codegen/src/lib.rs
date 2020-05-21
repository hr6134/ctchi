extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use syn::parse::{ParseStream, Parse};
use syn::token::Token;
use syn::export::Span;

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    let original_func = syn::parse_macro_input!(input as syn::ItemFn);

    let attrs = parse_macro_input!(args as AttributeArgs);

    let path = match &attrs[0] {
        NestedMeta::Lit(syn::Lit::Str(lit)) => Ok(lit.value()),
        _ => Err("No path"),
    }.unwrap();

    let original_func_ident = &original_func.sig.ident;
    let inputs = &original_func.sig.inputs;

    let mut fun_args = Vec::<Ident>::with_capacity(inputs.len());
    let mut fun_args_str = Vec::<String>::with_capacity(inputs.len());

    for i in inputs {
        let input_pat = match i {
            FnArg::Typed(PatType{attrs, pat, colon_token, ty}) => pat,
            _ => panic!(""),
        };

        if let syn::Pat::Ident(PatIdent { attrs, by_ref, mutability, ident, subpat }) = input_pat.as_ref() {
            fun_args.push(Ident::new(&ident.to_string(), Span::call_site()));
            fun_args_str.push(ident.to_string());
        }
    }

    let action_ident = Ident::new(
        &format!("ctchi_action_{}", original_func_ident),
        Span::call_site()
    );
    let routing_ident = Ident::new(
        &format!("ctchi_routing_{}", original_func_ident),
        Span::call_site()
    );

    let gen = quote! {
        #original_func

        fn #action_ident(url: &str) -> String {
            use regex::Regex;

            let url_replacer = Regex::new(r"\{(?P<first>.+?)\}").unwrap();
            let regex_url = url_replacer.replace_all(#path, "(?P<$first>.+?)");

            let parser = Regex::new(&regex_url).unwrap();

            #(
                let #fun_args = parser.captures(url).unwrap().name(#fun_args_str).unwrap().as_str();
            )*

            #original_func_ident(#(#fun_args),*)
        }

        fn #routing_ident() -> Route {
            Route {
                path: #path.to_string(),
                render_action: #action_ident,
            }
        }
    };

    gen.into()
}
