// Original work Copyright (c) 2014 The Rust Project Developers
// Modified work Copyright (c) 2016 Nikita Pekin and the lazycell contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

//! This crate provides a `LazyCell` struct which acts as a lazily filled
//! `Cell`, but with frozen contents.
//!
//! With a `RefCell`, the inner contents cannot be borrowed for the lifetime of
//! the entire object, but only of the borrows returned. A `LazyCell` is a
//! variation on `RefCell` which allows borrows to be tied to the lifetime of
//! the outer object.
//!
//! The limitation of a `LazyCell` is that after it is initialized, it can never
//! be modified.
//!
//! # Example
//!
//! The following example shows a quick example of the basic functionality of
//! `LazyCell`.
//!
//! ```
//! use lazycell::LazyCell;
//!
//! let lazycell = LazyCell::new();
//!
//! assert_eq!(lazycell.borrow(), None);
//! assert!(!lazycell.filled());
//! lazycell.fill(1);
//! assert!(lazycell.filled());
//! assert_eq!(lazycell.borrow(), Some(&1));
//! assert_eq!(lazycell.into_inner(), Some(1));
//! ```

use std::cell::RefCell;
use std::mem;

/// A lazily filled `Cell`, with frozen contents.
pub struct LazyCell<T> {
    inner: RefCell<Option<T>>,
}

impl<T> LazyCell<T> {
    /// Creates a new, empty, `LazyCell`.
    pub fn new() -> LazyCell<T> {
        LazyCell { inner: RefCell::new(None) }
    }

    /// Put a value into this cell.
    ///
    /// This function will fail if the cell has already been filled.
    pub fn fill(&self, t: T) {
        let mut slot = self.inner.borrow_mut();
        if slot.is_some() {
            panic!("lazy cell is already filled")
        }
        *slot = Some(t);
    }

    /// Test whether this cell has been previously filled.
    pub fn filled(&self) -> bool {
        self.inner.borrow().is_some()
    }

    /// Borrows the contents of this lazy cell for the duration of the cell
    /// itself.
    ///
    /// This function will return `Some` if the cell has been previously
    /// initialized, and `None` if it has not yet been initialized.
    pub fn borrow(&self) -> Option<&T> {
        match *self.inner.borrow() {
            Some(ref inner) => unsafe { Some(mem::transmute(inner)) },
            None => None,
        }
    }

    /// Consumes this `LazyCell`, returning the underlying value.
    pub fn into_inner(self) -> Option<T> {
        self.inner.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::LazyCell;

    #[test]
    fn test_borrow_from_empty() {
        let lazycell: LazyCell<usize> = LazyCell::new();

        let value = lazycell.borrow();
        assert_eq!(value, None);
    }

    #[test]
    fn test_fill_and_borrow() {
        let lazycell = LazyCell::new();

        assert!(!lazycell.filled());
        lazycell.fill(1);
        assert!(lazycell.filled());

        let value = lazycell.borrow();
        assert_eq!(value, Some(&1));
    }

    #[test]
    #[should_panic(expected = "lazy cell is already filled")]
    fn test_already_filled_panic() {
        let lazycell = LazyCell::new();

        lazycell.fill(1);
        lazycell.fill(1);
    }

    #[test]
    fn test_into_inner() {
        let lazycell = LazyCell::new();

        lazycell.fill(1);
        let value = lazycell.into_inner();
        assert_eq!(value, Some(1));
    }
}
