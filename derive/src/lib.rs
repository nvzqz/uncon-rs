//! Support for deriving traits found in [`uncon`].
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! uncon_derive = "1.1.0"
//! uncon = "1.1.0"
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
//! # #[macro_use] extern crate static_assertions;
//! # #[macro_use] extern crate uncon_derive;
//! # extern crate uncon;
//! # use uncon::*;
//! # macro_rules! assert_impl_from {
//! #    ($t:ty, $($u:ty),+) => {
//! #        assert_impl!($t, $(FromUnchecked<$u>, From<$u>),+)
//! #    }
//! # }
//! #[derive(FromUnchecked)]
//! struct U4 {
//!     bits: u8
//! }
//!
//! #[derive(FromUnchecked, PartialEq, Debug)]
//! #[uncon(impl_from, other(u16, u32, u64, usize))]
//! # #[uncon(other(i8, i16, i32, i64, isize))]
//! #[repr(u8)]
//! enum Flag {
//!     A, B, C, D
//! }
//!
//! // `usize` and `isize` also supported:
//! #[derive(FromUnchecked)]
//! #[repr(usize)]
//! enum Value {
//!     X, Y, Z
//! }
//!
//! # fn main() {
//! # assert_impl_from!(Flag, u8, u16, u32, u64, usize);
//! # assert_impl_from!(Flag, i8, i16, i32, i64, isize);
//! unsafe {
//!     let b = 0b1010;
//!     let x = U4::from_unchecked(b);
//!     assert_eq!(x.bits, b);
//!
//!     let n = 2u8;
//!     let f = Flag::from_unchecked(n);
//!     assert_eq!(f, Flag::C);
//!
//!     // Done via `#[uncon(other(u32, ...))]`
//!     assert_eq!(Flag::from_unchecked(n as u32), f);
//!
//!     // Done via `#[uncon(impl_from)]`
//!     assert_eq!(Flag::from(5usize), Flag::B);
//! }
//! # }
//! ```
//!
//! # Options
//!
//! - Derive [`FromUnchecked`] for other types:
//!   - Done via `#[uncon(other(...))]`.
//!   - Derives `FromUnchecked` with each type listed via an `as` cast to the
//!     inner or representative type.
//!
//! - Derive [`From`]:
//!   - Done via `#[uncon(from_impl)]`.
//!   - Only for C-like enums such that no variant is assigned a discriminant.
//!
//! [crate]: https://crates.io/crates/uncon_derive
//! [`uncon`]: https://docs.rs/uncon
//! [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
//! [`FromUnchecked`]: https://docs.rs/uncon/1.0.0/uncon/trait.FromUnchecked.html

#[macro_use]
extern crate quote;
extern crate proc_macro;
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

fn meta_items<'a, T: 'a>(items: T, ident: &str) -> Vec<&'a [NestedMetaItem]>
    where T: IntoIterator<Item=&'a MetaItem>
{
    items.into_iter().filter_map(|item| {
        if let MetaItem::List(ref id, ref items) = *item {
            if id == ident { return Some(items.as_ref()); }
        }
        None
    }).collect()
}

fn is_int_ty(s: &str) -> bool {
    let mut bytes = s.as_bytes();
    match bytes.get(0) {
        Some(&b'u') | Some(&b'i') => (),
        _ => return false,
    }
    bytes = &bytes[1..];
    match bytes.len() {
        0 => false,
        4 if bytes == b"size" => true,
        _ => {
            for &byte in bytes {
                if byte < b'0' || byte > b'9' {
                    return false;
                }
            }
            true
        },
    }
}

fn impl_from_unchecked(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let attr_items = |ident: &str| {
        meta_items(ast.attrs.iter().map(|a| &a.value), ident)
    };
    let uncon_items = attr_items("uncon");

    let core = if cfg!(feature = "std") { quote!(std) } else { quote!(core) };

    let impl_from = uncon_items.iter().flat_map(|i| i.iter()).filter_map(|item| {
        if let NestedMetaItem::MetaItem(MetaItem::Word(ref ident)) = *item {
            if ident == "impl_from" { return Some(true); }
        }
        None
    }).next().unwrap_or(false);

    let (ty, init, from_impl) = match ast.body {
        Body::Enum(ref variants) => {
            for variant in variants {
                assert!(!impl_from || variant.discriminant.is_none(),
                        "Cannot derive From due to {}::{} discriminant",
                        name, variant.ident);
                match variant.data {
                    VariantData::Unit => continue,
                    _ => panic!("Found non-unit variant '{}'", variant.ident),
                }
            }

            let items = *attr_items("repr").first().expect("Could not find `#[repr]` attribute");

            let repr = items.iter().filter_map(|ref item| {
                if let NestedMetaItem::MetaItem(ref item) = **item {
                    let name = item.name();
                    if is_int_ty(name) {
                        return Some(name);
                    }
                }
                None
            }).next().expect("Could not find integer repr for conversion");

            let init = quote! { ::#core::mem::transmute(inner) };
            let mut ty = Tokens::new();
            ty.append(repr);

            let from_impl = if impl_from {
                let num = variants.len();
                Some(quote! {
                    use uncon::IntoUnchecked;
                    unsafe { (inner % (#num as #ty)).into_unchecked() }
                })
            } else {
                None
            };

            (ty, init, from_impl)
        },
        Body::Struct(ref data) => {
            assert!(!impl_from, "Cannot derive From for non-enum types");

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
            (quote!(#ty), init, None)
        },
    };

    let mut other_items = Vec::<&NestedMetaItem>::new();

    for uncon_item in uncon_items.iter() {
        for other_item in meta_items(uncon_item.iter().filter_map(as_item), "other") {
            other_items.extend(other_item);
        }
    }

    let tys_impl = other_items.iter().filter_map(|item| {
        if let NestedMetaItem::MetaItem(MetaItem::Word(ref item)) = **item {
            let from_impl = from_impl.as_ref().map(|_| quote! {
                impl #impl_generics From<#item> for #name #ty_generics #where_clause {
                    #[inline]
                    fn from(inner: #item) -> Self { (inner as #ty).into() }
                }
            });
            Some(quote! {
                impl #impl_generics ::uncon::FromUnchecked<#item> for #name #ty_generics #where_clause {
                    #[inline]
                    unsafe fn from_unchecked(inner: #item) -> Self {
                        Self::from_unchecked(inner as #ty)
                    }
                }
                #from_impl
            })
        } else {
            None
        }
    });

    let from_impl = from_impl.as_ref().map(|fi| quote! {
        impl #impl_generics From<#ty> for #name #ty_generics #where_clause {
            #[inline]
            fn from(inner: #ty) -> Self { #fi }
        }
    });

    quote! {
        impl #impl_generics ::uncon::FromUnchecked<#ty> for #name #ty_generics #where_clause {
            #[inline]
            unsafe fn from_unchecked(inner: #ty) -> Self {
                #init
            }
        }
        #from_impl
        #(#tys_impl)*
    }
}
