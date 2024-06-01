// SPDX-License-Identifier: Apache-2.0

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Expr, ExprLit, Lit, Meta};

const UNSUPPORTED: &str = r#"only `/// ...` or `#[doc = "..."]` are supported"#;
const MISSING: &str = r#"missing doc attribute(s)"#;

#[proc_macro_derive(Doxed)]
pub fn derive_docs(input: TokenStream) -> TokenStream {
    // Parse the input.
    let input = parse_macro_input!(input as DeriveInput);

    // Get the type name and generics.
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Extract all the doc attributes from the input.
    let result: Result<Vec<_>, _> = input
        .attrs
        .iter()
        .filter_map(|attr| match attr {
            Attribute {
                meta: Meta::NameValue(nv),
                ..
            } if nv.path.is_ident("doc") => match &nv.value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => Some(Ok(s.value())),
                _ => Some(Err(UNSUPPORTED)),
            },
            _ => None,
        })
        .collect();

    let error = match result {
        // Handle error cases.
        Ok(dox) if dox.is_empty() => MISSING,
        Err(err) => err,

        // Generate the impl block.
        Ok(dox) => {
            let expanded = quote! {
                impl #impl_generics Doxed for #name #ty_generics #where_clause {
                    const DOX: &'static [&'static str] = &[#(#dox),*];
                }
            };

            return expanded.into();
        }
    };

    syn::Error::new_spanned(input.ident, error)
        .to_compile_error()
        .into()
}
