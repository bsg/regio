use std::sync::Mutex;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Stmt;

use lazy_static::lazy_static;

lazy_static! {
    static ref DEPS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[proc_macro_attribute]
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = match syn::parse::<syn::ItemFn>(item.clone()) {
        Ok(item_fn) => item_fn,
        Err(_) => return item,
    };

    let mut stmts: Vec<Stmt> = Vec::new();
    stmts.push(
        syn::parse::<syn::Stmt>(
            quote! {
                use regio::Component;
            }
            .into(),
        )
        .unwrap(),
    );
    stmts.push(
        syn::parse::<syn::Stmt>(
            quote! {
                let mut __registry = regio::Registry::new();
            }
            .into(),
        )
        .unwrap(),
    );
    for ident_str in DEPS.lock().unwrap().iter() {
        let ident = format_ident!("{}", ident_str);
        stmts.push(
            syn::parse::<syn::Stmt>(
                quote! {
                    __registry.put(<#ident>::new());
                }
                .into(),
            )
            .unwrap(),
        );
    }
    stmts.push(
        syn::parse::<syn::Stmt>(
            quote! {
                regio::init(__registry);
            }
            .into(),
        )
        .unwrap(),
    );
    stmts.append(&mut item_fn.block.stmts);
    item_fn.block.stmts = stmts;

    item_fn.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct =
        syn::parse::<syn::ItemStruct>(item.clone()).expect("Cannot use this macro here");
    let struct_ident = item_struct.ident;
    DEPS.lock().unwrap().push(struct_ident.to_string());
    item
}

#[proc_macro_attribute]
pub fn using(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = match syn::parse::<syn::ItemFn>(item.clone()) {
        Ok(item_fn) => item_fn,
        Err(_) => return item,
    };
    let attrs = attr.clone().into_iter().collect::<Vec<_>>();
    let ty = syn::parse_str::<syn::Ident>(&attrs.get(0).unwrap().to_string()).unwrap();
    // TODO attr #1 should be 'as'
    let var = syn::parse_str::<syn::Ident>(&attrs.get(2).unwrap().to_string()).unwrap();

    let mut stmts: Vec<Stmt> = Vec::new();
    stmts.push(
        syn::parse::<syn::Stmt>(
            quote!(
                let #var: &#ty = regio::get::<#ty>();
            )
            .into(),
        )
        .unwrap(),
    );
    stmts.append(&mut item_fn.block.stmts);
    item_fn.block.stmts = stmts;

    item_fn.into_token_stream().into()
}
