//! Procedural macros for floccus.
//! 
//! This crate contains procedural attribute macros (currently only one) used by the
//! [floccus](https://crates.io/crates/floccus). But potentially can be used in some
//! other crates, although will never be adapted to work with any other crate.
//! 
//! Source code of this crate is a modified copy of [derive-name-macros](https://crates.io/crates/derive-name-macros).

use proc_macro::TokenStream;
use quote::quote;
use syn::{self};

#[proc_macro_derive(Name)]
pub fn name(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    let ident = &ast.ident;
    let gen = quote! {
        impl crate::quantities::QuantityName for #ident {
            fn name() -> &'static str {
                stringify!(#ident)
            }
        }
    };
    gen.into()
}
