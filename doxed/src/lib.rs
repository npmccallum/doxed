// SPDX-License-Identifier: Apache-2.0

//! [![crates.io](https://img.shields.io/crates/v/doxed.svg)](https://crates.io/crates/doxed)
//! ![license](https://img.shields.io/badge/license-Apache%202.0-blue)
//! ![tests](https://github.com/npmccallum/doxed/actions/workflows/test.yml/badge.svg)
//! ![lints](https://github.com/npmccallum/doxed/actions/workflows/lint.yml/badge.svg)
//!
//! **Doxed** is a crate for making Rust doc strings available at runtime.
//!
//! This crate provides a trait, `Doxed`, which can be derived for any type to
//! make its doc strings available at runtime. The doc string is specified
//! using the `#[doc = "..."]` attribute or, more commonly, the Rust doc
//! comment (`///`).
//!
//! Note that when deriving `Doxed`, the doc string is not modified in any way.
//! This preserves the original formatting, including leading whitespace and
//! line breaks. If you want to do any processing on the doc string, you can
//! easily do so at runtime without additional derive magic.
//!
//! # Example
//!
//! ```rust
//! use doxed::Doxed;
//!
//! /// This is an example struct.
//! ///
//! /// Multiple lines are supported.
//! #[doc = "So are manual doc attributes."]
//! #[derive(Doxed)]
//! struct Example;
//!
//! assert_eq!(Example::DOX, &[
//!     " This is an example struct.",
//!     "",
//!     " Multiple lines are supported.",
//!     "So are manual doc attributes."
//! ]);
//! ```

#![no_std]

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use doxed_derive::Doxed;

/// A trait for types that have a doc string.
pub trait Doxed {
    /// The doc string(s) for the type.
    const DOX: &'static [&'static str];
}
