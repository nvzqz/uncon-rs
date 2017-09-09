#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate regex;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Body, MetaItem, NestedMetaItem, VariantData};
use quote::Tokens;

#[proc_macro_derive(FromUnchecked)]
pub fn from_unchecked(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    impl_from_unchecked(&ast).parse().unwrap()
}

fn impl_from_unchecked(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let core = if cfg!(feature = "std") { quote!(std) } else { quote!(core) };

    let (ty, init) = match ast.body {
        Body::Enum(ref variants) => {
            for variant in variants {
                match variant.data {
                    VariantData::Unit => continue,
                    _ => panic!("Found non-unit variant '{}'", variant.ident),
                }
            }

            let items = ast.attrs.iter().filter_map(|a| {
                if let MetaItem::List(ref ident, ref items) = a.value {
                    if ident == "repr" {
                        return Some(items);
                    }
                }
                None
            }).next().expect("Could not find `#[repr]` attribute");

            let int_ty = regex::Regex::new("^(i|u)\\d+$").unwrap();

            let repr = items.iter().filter_map(|item| {
                if let NestedMetaItem::MetaItem(ref item) = *item {
                    let name = item.name();
                    if int_ty.is_match(name) {
                        return Some(name);
                    }
                }
                None
            }).next().expect("Could not find integer repr for conversion");

            let init = quote! { ::#core::mem::transmute(inner) };
            let mut ty = Tokens::new();
            ty.append(repr);

            (ty, init)
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
            (quote!(#ty), init)
        },
    };
    quote! {
        impl #impl_generics ::unchecked_convert::FromUnchecked<#ty> for #name #ty_generics #where_clause {
            #[inline]
            unsafe fn from_unchecked(inner: #ty) -> Self {
                #init
            }
        }
    }
}
