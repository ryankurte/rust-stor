//! [`Stor`] trait defines underlying storage for use in type definitions
//! 
//! ```
//! use stor::Stor;
//! 
//! /// Generic object over storage types
//! enum Something<S: Stor<()>> {
//!   List(S::List),
//!   Str(S::String),
//!   Bytes(S::Bytes),
//! }
//! 
//! /// Owned version (requires `alloc` feature)
//! type SomethingOwned = Something<stor::Owned>;
//! let _ = SomethingOwned::Str("hello".to_string());
//! let _ = SomethingOwned::Bytes(vec![0xaa, 0xbb, 0xcc]);
//! 
//! /// Reference version
//! type SomethingRef<'a> = Something<stor::Ref<'a>>;
//! let _ = SomethingRef::Str("hello");
//! let _ = SomethingRef::Bytes(&[0xaa, 0xbb, 0xcc]);
//! 
//! /// Const N version
//! type SomethingConst<'a> = Something<stor::Const<3>>;
//! let _ = SomethingConst::Str("hello");
//! let _ = SomethingConst::Bytes([0xaa, 0xbb, 0xcc]);
//! ```
//! 
#![no_std]

use core::fmt::Debug;
use core::marker::PhantomData;

#[cfg(feature = "alloc")]
extern crate alloc;

/// [`Stor`] trait provides abstract container types
pub trait Stor<Inner: Debug = ()>: Debug {
    /// Type for holding lists of Inner objects
    type List: AsRef<[Inner]> + Debug;
    /// Type for holding strings
    type String: AsRef<str> + Debug;
    /// Type for holding bytes
    type Bytes: AsRef<[u8]> + Debug;
}

/// Owned marker uses [`alloc`] backed storage
#[cfg(feature = "alloc")]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Owned;

#[cfg(feature = "alloc")]
impl <T: Clone + Debug> Stor<T> for Owned {
    type List = alloc::vec::Vec<T>;
    type String = alloc::string::String;
    type Bytes = alloc::vec::Vec<u8>;
}

/// Ref marker uses `&'a T` containers
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ref<'a> (PhantomData<&'a ()>);

impl <'a, T: Clone + Debug + 'a> Stor<T> for Ref<'a> {
    type List = &'a [T];
    type String = &'a str;
    type Bytes = &'a [u8];
}

/// Const marker uses const size containers
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Const<const N: usize>;

impl <T: Clone + Debug, const N: usize> Stor<T> for Const<N> {
    type List = [T; N];
    type String = &'static str;
    type Bytes = [u8; N];
}

/// Heapless marker uses [`heapless`] containers
#[cfg(feature = "heapless")]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Heapless<const N: usize>;

#[cfg(feature = "heapless")]
impl <T: Clone + Debug, const N: usize> Stor<T> for Heapless<N> {
    type List = heapless::Vec<T, N>;
    type String = heapless::String<N>;
    type Bytes = heapless::Vec<u8, N>;
}

#[cfg(test)]
mod tests {
    // TODO: write some tests
}
