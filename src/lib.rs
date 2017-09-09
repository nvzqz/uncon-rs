//! Traits for unchecked conversions between types.

#![no_std]

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
