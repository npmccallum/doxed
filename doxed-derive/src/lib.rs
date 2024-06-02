// SPDX-License-Identifier: Apache-2.0

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{Attribute, DeriveInput, Error, Expr, ExprLit, Lit, Meta};

const UNSUPPORTED: &str = r#"only `/// ...` or `#[doc = "..."]` are supported"#;

fn filter_attr(attr: &Attribute) -> Option<Result<String, Error>> {
    match &attr.meta {
        Meta::NameValue(nv) if nv.path.is_ident("doc") => match &nv.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => Some(Ok(s.value())),
            _ => Some(Err(Error::new_spanned(attr, UNSUPPORTED))),
        },
        _ => None,
    }
}

fn inner(input: DeriveInput) -> Result<TokenStream, Error> {
    // Extract all the doc attributes from the input.
    let dox = input
        .attrs
        .iter()
        .filter_map(filter_attr)
        .collect::<Result<Vec<String>, _>>()?;

    // Generate the implementation.
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(From::from(quote::quote! {
        impl #impl_generics Doxed for #name #ty_generics #where_clause {
            const DOX: &'static [&'static str] = &[#(#dox),*];
        }
    }))
}

#[proc_macro_derive(Doxed)]
pub fn derive_dox(input: TokenStream) -> TokenStream {
    inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error().into())
}
