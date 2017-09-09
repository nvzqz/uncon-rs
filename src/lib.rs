//! Traits for unchecked conversions between types.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

use core::str;

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

#[cfg(feature = "std")]
impl FromUnchecked<Vec<u8>> for String {
    #[inline]
    unsafe fn from_unchecked(utf8: Vec<u8>) -> String {
        String::from_utf8_unchecked(utf8)
    }
}

#[cfg(feature = "std")]
impl FromUnchecked<Box<[u8]>> for String {
    #[inline]
    unsafe fn from_unchecked(utf8: Box<[u8]>) -> String {
        utf8.into_vec().into_unchecked()
    }
}
