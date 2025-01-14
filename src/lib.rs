// SPDX-FileCopyrightText: © 2025 AMD <nathaniel.mccallum@amd.com>
// SPDX-License-Identifier: MIT

//! A no_std, fixed-size string buffer for embedded systems and performance-critical contexts.
//!
//! # Overview
//!
//! This crate provides [`StrBuf`], a fixed-capacity string buffer that operates without
//! heap allocation. It is designed for use in embedded systems, `no_std` environments,
//! and performance-critical code paths.
//!
//! # Features
//! - Fixed-size allocation with runtime size checking
//! - Standard formatting traits ([`core::fmt::Write`], [`core::fmt::Display`])
//! - Zero heap allocations
//! - Zero unsafe code
//! - Efficient string building operations
//!
//! # Usage
//! ```rust
//! use strbuf::StrBuf;
//! use core::fmt::Write;
//!
//! // Basic formatting
//! let mut buf = StrBuf::<64>::default();
//! write!(buf, "Temperature: {:.1}°C", 23.5).unwrap();
//! assert_eq!(buf.as_ref(), "Temperature: 23.5°C");
//!
//! // Error handling
//! let result = StrBuf::<4>::display("Too long");
//! assert!(result.is_err());
//! ```
//!
//! # Technical Details
//! - Uses const generics for compile-time size specification
//! - Maintains UTF-8 validity
//! - Implements common traits: [`core::ops::Deref`], [`core::convert::AsRef`]
//! - Thread-safe (no interior mutability)
//!
//! [`StrBuf`]: struct.StrBuf.html

#![no_std]
#![forbid(unsafe_code, clippy::expect_used, clippy::panic)]
#![deny(
    clippy::all,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    single_use_lifetimes,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unstable_features,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

use core::{fmt::Write, ops::Deref};

/// A fixed-size buffer for writing strings without heap allocation.
///
/// `StrBuf<N>` provides a fixed-size buffer of N bytes that can be used for string
/// formatting operations in no_std environments or when heap allocation is undesirable.
///
/// # Capacity
///
/// The buffer size N is specified as a const generic parameter. Attempting to write
/// beyond this size will return an error.
///
/// # Example
/// ```rust
/// use strbuf::StrBuf;
/// use core::fmt::Write;
///
/// let mut buf = StrBuf::<128>::default();
/// write!(buf, "Hello, {}!", "world").unwrap();
/// assert_eq!(buf.as_ref(), "Hello, world!");
/// ```
///
/// # Implementation
///
/// The buffer maintains an internal cursor position and validates all writes to ensure
/// UTF-8 correctness and capacity constraints.
#[derive(Copy, Clone, Debug)]
pub struct StrBuf<const N: usize>([u8; N], usize);

impl<const N: usize> Default for StrBuf<N> {
    fn default() -> Self {
        Self([0; N], 0)
    }
}

impl<const N: usize> Deref for StrBuf<N> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        core::str::from_utf8(&self.0[..self.1]).unwrap()
    }
}

impl<const N: usize> AsRef<str> for StrBuf<N> {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl<const N: usize> Write for StrBuf<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let (.., free) = self.0.split_at_mut(self.1);
        if s.len() > free.len() {
            return Err(core::fmt::Error);
        }

        let (into, ..) = free.split_at_mut(s.len());
        into.copy_from_slice(s.as_bytes());
        self.1 += s.len();
        Ok(())
    }
}

impl<const N: usize> StrBuf<N> {
    /// Creates a new buffer containing the formatted string representation of a value.
    ///
    /// # Arguments
    ///
    /// * `value` - Any value that implements `Display`
    ///
    /// # Errors
    ///
    /// Returns `core::fmt::Error` if formatting fails or if the buffer is too small.
    pub fn display<T: core::fmt::Display>(value: T) -> Result<StrBuf<N>, core::fmt::Error> {
        let mut buf = StrBuf::default();
        write!(buf, "{}", value)?;
        Ok(buf)
    }

    /// Creates a new buffer containing the result of the formatted arguments.
    ///
    /// This method is particularly useful when working with `format_args!()` macro
    /// or when implementing custom formatting traits.
    ///
    /// # Arguments
    ///
    /// * `args` - Format arguments created via `format_args!()` macro
    ///
    /// # Errors
    ///
    /// Returns `core::fmt::Error` if formatting fails or if the buffer is too small.
    ///
    /// # Example
    ///
    /// ```rust
    /// use strbuf::StrBuf;
    ///
    /// let name = "world";
    /// let buf = StrBuf::<128>::format(format_args!("Hello, {}!", name)).unwrap();
    /// assert_eq!(buf.as_ref(), "Hello, world!");
    /// ```
    pub fn format(args: core::fmt::Arguments<'_>) -> Result<StrBuf<N>, core::fmt::Error> {
        let mut buf = StrBuf::default();
        buf.write_fmt(args)?;
        Ok(buf)
    }
}
