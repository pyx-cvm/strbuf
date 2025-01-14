# strbuf

A no_std, fixed-size string buffer for embedded systems and performance-critical contexts.

## Overview

This crate provides [`StrBuf`], a fixed-capacity string buffer that operates without
heap allocation. It is designed for use in embedded systems, `no_std` environments,
and performance-critical code paths.

## Features
- Fixed-size allocation with runtime size checking
- Standard formatting traits ([`core::fmt::Write`], [`core::fmt::Display`])
- Zero heap allocations
- Zero unsafe code
- Efficient string building operations

## Usage
```rust
use strbuf::StrBuf;
use core::fmt::Write;

// Basic formatting
let mut buf = StrBuf::<64>::default();
write!(buf, "Temperature: {:.1}°C", 23.5).unwrap();
assert_eq!(buf.as_ref(), "Temperature: 23.5°C");

// Error handling
let result = StrBuf::<4>::display("Too long");
assert!(result.is_err());
```

## Technical Details
- Uses const generics for compile-time size specification
- Maintains UTF-8 validity
- Implements common traits: [`core::ops::Deref`], [`core::convert::AsRef`]
- Thread-safe (no interior mutability)

[`StrBuf`]: struct.StrBuf.html
