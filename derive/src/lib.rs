//! Support for deriving traits found in [`uncon`].
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! uncon_derive = "1.0.2"
//! uncon = "1.0.0"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! #[macro_use]
//! extern crate uncon_derive;
//! extern crate uncon;
//! # fn main() {}
//! ```
//!
//! # Examples
//!
//! The [`FromUnchecked`] trait can be derived for:
//!
//! - Structs with a single field
//! - C-like enums with `#[repr]` attribute
//!
//! ```
//! # extern crate core;
//! # #[macro_use]
//! # extern crate uncon_derive;
//! # extern crate uncon;
//! # use uncon::*;
//! #[derive(FromUnchecked)]
//! struct U4 { bits: u8 }
//!
//! #[derive(FromUnchecked)]
//! #[uncon(other(u16, u32, u64, usize))]
//! #[repr(u8)]
//! enum Flag {
//!     A, B, C, D
//! }
//!
//! // `usize` and `isize` also supported:
//! #[derive(FromUnchecked)]
//! #[repr(usize)]
//! enum Value { X, Y, Z }
//!
//! # fn main() {
//! unsafe {
//!     let b = 0b1010;
//!     let x = U4::from_unchecked(b);
//!     assert_eq!(x.bits, b);
//!
//!     let n = 2;
//!     let f = Flag::from_unchecked(n);
//!     assert_eq!(f as u8, n);
//!
//!     // Done via `#[uncon(other(u32, ...))]`
//!     let f = Flag::from_unchecked(n as u32);
//! }
//! # }
//! ```
//!
//! [crate]: https://crates.io/crates/uncon_derive
//! [`uncon`]: https://docs.rs/uncon
//! [`FromUnchecked`]: https://docs.rs/uncon/1.0.0/uncon/trait.FromUnchecked.html

#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate regex;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Body, MetaItem, NestedMetaItem, VariantData};
use quote::Tokens;

#[doc(hidden)]
#[proc_macro_derive(FromUnchecked, attributes(uncon))]
pub fn from_unchecked(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    impl_from_unchecked(&ast).parse().unwrap()
}

fn as_item(item: &NestedMetaItem) -> Option<&MetaItem> {
    if let NestedMetaItem::MetaItem(ref item) = *item {
        Some(item)
    } else {
        None
    }
}

fn meta_items<'a, T: 'a>(items: T, ident: &str) -> Option<&'a [NestedMetaItem]>
    where T: IntoIterator<Item=&'a MetaItem>
{
    items.into_iter().filter_map(|item| {
        if let MetaItem::List(ref id, ref items) = *item {
            if id == ident { return Some(items.as_ref()); }
        }
        None
    }).next()
}

fn impl_from_unchecked(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let attr_items = |ident: &str| {
        meta_items(ast.attrs.iter().map(|a| &a.value), ident)
    };

    let core = if cfg!(feature = "std") { quote!(std) } else { quote!(core) };

    let (ty, init) = match ast.body {
        Body::Enum(ref variants) => {
            for variant in variants {
                match variant.data {
                    VariantData::Unit => continue,
                    _ => panic!("Found non-unit variant '{}'", variant.ident),
                }
            }

            let items = attr_items("repr").expect("Could not find `#[repr]` attribute");
            let int_ty = regex::Regex::new("^(i|u)(\\d+|size)$").unwrap();

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

    let mut tys_impl = Vec::<Tokens>::new();

    if let Some(items) = attr_items("uncon") {
        if let Some(items) = meta_items(items.iter().filter_map(as_item), "other") {
            let items = items.iter().filter_map(|item| {
                if let NestedMetaItem::MetaItem(MetaItem::Word(ref ident)) = *item {
                    Some(ident)
                } else {
                    None
                }
            });
            tys_impl.extend(items.map(|item| quote! {
                impl #impl_generics ::uncon::FromUnchecked<#item> for #name #ty_generics #where_clause {
                    unsafe fn from_unchecked(inner: #item) -> Self {
                        Self::from_unchecked(inner as #ty)
                    }
                }
            }));
        }
    }

    quote! {
        impl #impl_generics ::uncon::FromUnchecked<#ty> for #name #ty_generics #where_clause {
            #[inline]
            unsafe fn from_unchecked(inner: #ty) -> Self {
                #init
            }
        }

        #(#tys_impl)*
    }
}
