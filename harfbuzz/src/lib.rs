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

mod buffer;
pub use self::buffer::{Buffer, BufferFlags};

mod direction;
pub use self::direction::Direction;

mod language;
pub use self::language::Language;

mod blob;
pub use self::blob::Blob;

mod face;
pub use self::face::Face;

mod font;
pub use self::font::Font;

mod font_extents;
pub use self::font_extents::FontExtents;

mod feature;
pub use self::feature::Feature;

mod user_data;
pub use self::user_data::UserDataKey;

mod tag;
pub use self::tag::Tag;

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
