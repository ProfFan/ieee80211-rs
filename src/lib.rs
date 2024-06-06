#![no_std]
#![forbid(unsafe_code)]
#![deny(unused)]
//! # ieee80211-rs
//! This is a library for dealing with IEEE 802.11(aka. WiFi) frames.
//! This includes reading and writing, but may in the future also include crypto primitives and other things related to WiFi frames.
//! The library doesn't use allocations and instead of collections, there is heavy use of [Iterators](Iterator).
//! If the library seems like a mess of generics, than that's absolutely correct.
//! Generics allow us, to make the library comfortable to use.
//! ## Parsing
//! We internally rely on [scroll] for parsing, which is therefore re-exported.
//! To get started see the `examples` folder.
//! ## Docs
//! Sometimes the explanation for fields seems like it was ripped strait from the standard, which is exactly what happened here.
//! They maybe slightly altered though.
//! If some doc comments remind you of [this](https://i2-prod.mirror.co.uk/incoming/article5875284.ece/ALTERNATES/s1200c/Stop-sign.jpg), then you're not alone.

#[cfg(feature = "alloc")]
extern crate alloc;

/// This is a collection of commonly used types.
pub mod common;
/// This module contains the elements, which are found in the body of some frames.
/// If an element only consists of one struct, like the [SSID](elements::SSIDElement), they are re-exported, otherwise they get their own module.
pub mod elements;
mod frames;

pub use frames::*;

pub use const_soft_float;
pub use macro_bits;
pub use scroll;
