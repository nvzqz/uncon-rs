//! Traits for **un**checked **con**versions between types.
//!
//! The purpose of this crate is to provide [`FromUnchecked`] and
//! [`IntoUnchecked`], which can be used across different crates to allow for
//! fast conversions between types when speed is necessary. These traits allow
//! for code to be much more expressive than when using `mem::transmute`. They
//! are the unchecked/unsafe equivalents of [`From`] and [`Into`], respectively.
//!
//! [Some types](trait.FromUnchecked.html#implementors) already implement
//! [`FromUnchecked`] out-of-the-box.
//!
//! [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
//! [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! uncon = "1.0.0"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate uncon;
//! # fn main() {}
//! ```
//!
//! # Examples
//!
//! A type may wrap around another but must maintain certain invariants that
//! aren't met by the inner type. An example of this is `str` in relation to
//! `[u8]` where a string is just a UTF-8 encoded byte slice.
//!
//! In this example, `U4` is a simple wrapper around `u8` where valid instances
//! must only ever have 4 bits set.
//!
//! ```
//! # use uncon::*;
//! struct U4(u8);
//!
//! impl From<u8> for U4 {
//!     fn from(byte: u8) -> U4 {
//!         U4(byte & 0b1111)
//!     }
//! }
//!
//! impl FromUnchecked<u8> for U4 {
//!     unsafe fn from_unchecked(byte: u8) -> U4 {
//!         U4(byte)
//!     }
//! }
//! ```
//!
//! If a type `T` implements [`FromUnchecked`] for some type `U`, then `U`
//! automatically implements [`IntoUnchecked`] for `T`.
//!
//! ```
//! # use uncon::*;
//! let b = [b'h', b'i'];
//! let s: &str = unsafe { b.as_ref().into_unchecked() };
//!
//! assert_eq!(s, "hi");
//! ```
//!
//! # Deriving Traits
//!
//! See the docs of [`uncon_derive`] for info on deriving this crate's traits.
//!
//! ```rust,ignore
//! #[derive(FromUnchecked)]
//! struct Flags(u8);
//!
//! let f = unsafe { Flags::from_unchecked(0b1100) };
//! ```
//!
//! # Safety
//!
//! - `Vec<U>` to `Vec<T>`, `Box<U>` to `Box<T>` and `&U` to `&T` conversions
//!   are similar to [`mem::transmute`] except without the
//!   [undefined behavior][ub]. There are absolutely **_no_** safety measures.
//!   - These conversions are extremely unsafe and should only be done in cases
//!     such as turning `Vec<i8>` into `Vec<u8>` or something similarly trivial.
//!   - If `T` implements `Drop` in the case of `Vec<T>`, consider `map`ping
//!     `from_unchecked` and `collect`ing the results.
//!
//! [crate]: https://crates.io/crates/uncon
//! [ub]: https://en.wikipedia.org/wiki/Undefined_behavior
//! [`mem::transmute`]: https://doc.rust-lang.org/std/mem/fn.transmute.html
//! [`FromUnchecked`]: trait.FromUnchecked.html
//! [`IntoUnchecked`]: trait.IntoUnchecked.html
//! [`uncon_derive`]: https://docs.rs/uncon_derive

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "alloc", feature(alloc))]

#[cfg(feature = "std")]
extern crate core;
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::{String, Vec};

#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::rc::Rc;

#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::arc::Arc;

use core::{mem, slice, str};

/// Unchecked and potentially unsafe conversions from `T` into `Self`.
pub trait FromUnchecked<T>: Sized {
    /// Performs the unchecked conversion.
    unsafe fn from_unchecked(T) -> Self;
}

/// Unchecked and potentially unsafe conversions from `Self` into `T`.
pub trait IntoUnchecked<T>: Sized {
    /// Performs the unchecked conversion.
    unsafe fn into_unchecked(self) -> T;
}

impl<T, U: FromUnchecked<T>> IntoUnchecked<U> for T {
    #[inline]
    unsafe fn into_unchecked(self) -> U {
        U::from_unchecked(self)
    }
}

impl<'a, T, U> FromUnchecked<&'a U> for &'a T {
    #[inline]
    unsafe fn from_unchecked(other: &U) -> &T {
        &*(other as *const U as *const T)
    }
}

impl<'a, T, U> FromUnchecked<&'a mut U> for &'a mut T {
    #[inline]
    unsafe fn from_unchecked(other: &mut U) -> &mut T {
        &mut *(other as *mut U as *mut T)
    }
}

impl<'a, T: ?Sized> FromUnchecked<*const T> for &'a T {
    #[inline]
    unsafe fn from_unchecked(ptr: *const T) -> &'a T {
        &*ptr
    }
}

impl<'a, T: ?Sized> FromUnchecked<*mut T> for &'a mut T {
    #[inline]
    unsafe fn from_unchecked(ptr: *mut T) -> &'a mut T {
        &mut *ptr
    }
}

impl<'a, T, U> FromUnchecked<&'a [U]> for &'a [T] {
    #[inline]
    unsafe fn from_unchecked(slice: &[U]) -> &[T] {
        slice::from_raw_parts(slice.as_ptr() as _, slice.len())
    }
}

impl<'a, T, U> FromUnchecked<&'a mut [U]> for &'a mut [T] {
    #[inline]
    unsafe fn from_unchecked(slice: &mut [U]) -> &mut [T] {
        slice::from_raw_parts_mut(slice.as_mut_ptr() as _, slice.len())
    }
}

impl<'a> FromUnchecked<&'a [u8]> for &'a str {
    #[inline]
    unsafe fn from_unchecked(utf8: &[u8]) -> &str {
        str::from_utf8_unchecked(utf8)
    }
}

impl<'a> FromUnchecked<&'a mut [u8]> for &'a mut str {
    #[inline]
    unsafe fn from_unchecked(utf8: &mut [u8]) -> &mut str {
        str::from_utf8_unchecked_mut(utf8)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, U> FromUnchecked<Vec<U>> for Vec<T> {
    #[inline]
    unsafe fn from_unchecked(mut vec: Vec<U>) -> Vec<T> {
        let new = Vec::from_raw_parts(
            vec.as_mut_ptr() as _,
            vec.len(),
            vec.capacity()
        );
        mem::forget(vec);
        new
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl FromUnchecked<Vec<u8>> for String {
    #[inline]
    unsafe fn from_unchecked(utf8: Vec<u8>) -> Self {
        Self::from_utf8_unchecked(utf8)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl FromUnchecked<Box<[u8]>> for String {
    #[inline]
    unsafe fn from_unchecked(utf8: Box<[u8]>) -> Self {
        utf8.into_vec().into_unchecked()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, U> FromUnchecked<Box<U>> for Box<T> {
    #[inline]
    unsafe fn from_unchecked(b: Box<U>) -> Self {
        Box::from_raw(Box::into_raw(b) as _)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, U> FromUnchecked<Box<[U]>> for Box<[T]> {
    #[inline]
    unsafe fn from_unchecked(b: Box<[U]>) -> Self {
        Box::from_raw(Box::into_raw(b) as _)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: ?Sized> FromUnchecked<*mut T> for Box<T> {
    #[inline]
    unsafe fn from_unchecked(ptr: *mut T) -> Self {
        Self::from_raw(ptr)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> FromUnchecked<*const T> for Arc<T> {
    #[inline]
    unsafe fn from_unchecked(ptr: *const T) -> Self {
        Self::from_raw(ptr)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, U> FromUnchecked<Arc<U>> for Arc<T> {
    #[inline]
    unsafe fn from_unchecked(arc: Arc<U>) -> Self {
        Arc::from_raw(Arc::into_raw(arc) as _)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> FromUnchecked<*const T> for Rc<T> {
    #[inline]
    unsafe fn from_unchecked(ptr: *const T) -> Self {
        Self::from_raw(ptr)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, U> FromUnchecked<Rc<U>> for Rc<T> {
    #[inline]
    unsafe fn from_unchecked(rc: Rc<U>) -> Self {
        Rc::from_raw(Rc::into_raw(rc) as _)
    }
}
