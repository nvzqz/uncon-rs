#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::Body;

#[proc_macro_derive(FromUnchecked)]
pub fn from_unchecked(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    impl_from_unchecked(&ast).parse().unwrap()
}

fn impl_from_unchecked(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    match ast.body {
        Body::Enum(_) => {
            // Derive for C-like enums
            unimplemented!();
        },
        Body::Struct(ref data) => {
            let fields = data.fields();
            if fields.len() != 1 {
                panic!("`FromUnchecked` can only be derived for types with a single field");
            }
            let field = &fields[0];

            let init = if let Some(ref ident) = field.ident {
                quote! { #name { #ident: inner } }
            } else {
                quote! { #name(inner) }
            };

            let ty = &field.ty;

            quote! {
                impl #impl_generics ::unchecked_convert::FromUnchecked<#ty> for #name #ty_generics #where_clause {
                    #[inline]
                    unsafe fn from_unchecked(inner: #ty) -> Self {
                        #init
                    }
                }
            }
        },
    }
}
