// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! HarfBuzz is a text shaping engine. It solves the problem of selecting
//! and positioning glyphs from a font given a Unicode string.

#![warn(missing_docs)]
#![deny(
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

pub extern crate harfbuzz_sys as sys;

#[macro_use]
mod user_data;

mod blob;
mod buffer;
mod direction;
mod errors;
mod face;
mod feature;
mod font;
mod font_extents;
mod language;
mod tag;

pub use blob::Blob;
pub use buffer::{Buffer, BufferFlags};
pub use direction::Direction;
pub use errors::Error;
pub use face::Face;
pub use feature::Feature;
pub use font::Font;
pub use font_extents::FontExtents;
pub use language::Language;
pub use tag::Tag;
pub use user_data::UserDataKey;

/// Data type for holding Unicode codepoints. Also used to hold glyph IDs.
pub type Codepoint = sys::hb_codepoint_t;

/// Shapes buffer using font turning its Unicode characters content to positioned glyphs.
/// If features is not NULL, it will be used to control the features applied during shaping.
/// If two features have the same tag but overlapping ranges the value of the feature
/// with the higher index takes precedence.
pub fn shape(font: &Font, buf: &mut Buffer, features: &[Feature]) {
    unsafe {
        sys::hb_shape(
            font.as_ptr() as *mut sys::hb_font_t,
            buf.as_mut_ptr(),
            features.as_ptr() as *const _,
            features.len() as u32,
        );
    }
}
