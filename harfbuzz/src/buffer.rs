// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use sys;

use crate::{Codepoint, Direction, Language};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct GlyphInfo(sys::hb_glyph_info_t);
impl GlyphInfo {
    pub fn codepoint(&self) -> u32 {
        self.0.codepoint
    }
    pub fn cluster(&self) -> u32 {
        self.0.cluster
    }
    pub fn mask(&self) -> u32 {
        self.0.mask
    }
}

impl std::fmt::Debug for GlyphInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("GlyphInfo")
            .field("codepoint", &self.0.codepoint)
            .field("cluster", &self.0.cluster)
            .field("mask", &self.0.mask)
            // .field("var1", &self.0.var1)
            // .field("var2", &self.0.var2)
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct GlyphPosition(sys::hb_glyph_position_t);
impl GlyphPosition {
    pub fn x_advance(&self) -> i32 {
        self.0.x_advance
    }
    pub fn y_advance(&self) -> i32 {
        self.0.y_advance
    }
    pub fn x_offset(&self) -> i32 {
        self.0.x_offset
    }
    pub fn y_offset(&self) -> i32 {
        self.0.y_offset
    }
}

impl std::fmt::Debug for GlyphPosition {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("GlyphPosition")
            .field("x_advance", &self.0.x_advance)
            .field("y_advance", &self.0.y_advance)
            .field("x_offset", &self.0.x_offset)
            .field("y_offset", &self.0.y_offset)
            .finish()
    }
}

/// A series of Unicode characters.
///
/// ## Adding Text
///
/// Since in Rust, a value of type `&str` must contain valid UTF-8
/// text, adding text to a `Buffer` is simple:
///
/// ```
/// # use harfbuzz::Buffer;
/// let mut b = Buffer::new();
/// b.add_str("Hello World", 0, None);
/// assert_eq!(b.is_empty(), false);
/// ```
///
/// or, more simply:
///
/// ```
/// # use harfbuzz::Buffer;
/// let b = Buffer::with("Hello World");
/// assert_eq!(b.is_empty(), false);
/// ```
///
/// ## Segment Properties
///
/// In addition to the text itself, there are three important properties
/// that influence how a piece of text is shaped:
///
/// * Direction: The direction in which the output glyphs flow. This is
///   typically left to right or right to left. This is controlled via
///   the [`set_direction`] method on `Buffer`.
/// * Script: Script is crucial for choosing the proper shaping behaviour
///   for scripts that require it (e.g. Arabic) and the which OpenType
///   features defined in the font to be applied. This is controlled via
///   the [`set_script`] method on `Buffer`.
/// * Language: Languages are crucial for selecting which OpenType feature
///   to apply to the buffer which can result in applying language-specific
///   behaviour. Languages are orthogonal to the scripts, and though they
///   are related, they are different concepts and should not be confused
///   with each other. This is controlled via the [`set_language`] method
///   on `Buffer`.
///
/// Additionally, Harfbuzz can attempt to infer the values for these
/// properties using the [`guess_segment_properties`] method on `Buffer`:
///
/// ```
/// # use harfbuzz::{Buffer, Direction, sys};
/// let mut b = Buffer::with("مساء الخير");
/// b.guess_segment_properties();
/// assert_eq!(b.get_direction(), Direction::RTL);
/// assert_eq!(b.get_script(), sys::HB_SCRIPT_ARABIC);
/// ```
///
/// [`set_direction`]: #method.set_direction
/// [`set_script`]: #method.set_script
/// [`set_language`]: #method.set_language
/// [`guess_segment_properties`]: #method.guess_segment_properties
pub struct Buffer {
    /// The underlying `hb_buffer_t` from the `harfbuzz-sys` crate.
    ///
    /// This isn't commonly needed unless interfacing directly with
    /// functions from the `harfbuzz-sys` crate that haven't been
    /// safely exposed.
    raw: *mut sys::hb_buffer_t,
}

impl Buffer {
    /// Create a new, empty buffer.
    ///
    /// ```
    /// # use harfbuzz::Buffer;
    /// let b = Buffer::new();
    /// assert!(b.is_empty());
    /// ```
    pub fn new() -> Self {
        Buffer::default()
    }

    /// Construct a `Buffer` from a raw pointer. Takes ownership of the buffer.
    pub unsafe fn from_raw(raw: *mut sys::hb_buffer_t) -> Self {
        Buffer { raw }
    }

    /// Borrows a raw pointer to the buffer.
    pub fn as_ptr(&self) -> *const sys::hb_buffer_t {
        self.raw
    }

    pub fn as_mut_ptr(&self) -> *mut sys::hb_buffer_t {
        self.raw
    }

    /// Gives up ownership and returns a raw pointer to the buffer.
    pub fn into_raw(self) -> *mut sys::hb_buffer_t {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    /// Create a new buffer with the given text.
    pub fn with(text: &str) -> Self {
        let mut b = Buffer::new();
        b.add_str(text, 0, None);
        b
    }

    /// Create a new, empty buffer with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut b = Buffer::default();
        b.reserve(capacity);
        b
    }

    /// Add UTF-8 encoded text to the buffer.
    pub fn add_str(&mut self, text: &str, start_at: usize, length: Option<usize>) {
        unsafe {
            sys::hb_buffer_add_utf8(
                self.raw,
                text.as_ptr() as *const std::os::raw::c_char,
                text.len() as std::os::raw::c_int,
                start_at as std::os::raw::c_uint,
                length.map(|l| l as std::os::raw::c_int).unwrap_or(-1),
            )
        };
    }

    /// Append part of the contents of another buffer to this one.
    ///
    /// ```
    /// # use harfbuzz::Buffer;
    /// let mut b1 = Buffer::with("butter");
    /// let b2 = Buffer::with("fly");
    /// b1.append(&b2, 0, 3);
    /// assert_eq!(b1.len(), "butterfly".len());
    /// ```
    pub fn append(&mut self, other: &Buffer, start: usize, end: usize) {
        unsafe {
            sys::hb_buffer_append(
                self.raw,
                other.raw,
                start as std::os::raw::c_uint,
                end as std::os::raw::c_uint,
            )
        };
    }

    /// Throw away text stored in the buffer, but maintain the
    /// currently configured Unicode functions and flags.
    ///
    /// Text, glyph info, and segment properties will be discarded.
    pub fn clear_contents(&mut self) {
        unsafe { sys::hb_buffer_clear_contents(self.raw) };
    }

    /// Throw away all data stored in the buffer as well as configuration
    /// parameters like Unicode functions, flags, and segment properties.
    pub fn reset(&mut self) {
        unsafe { sys::hb_buffer_reset(self.raw) };
    }

    /// Preallocate space to fit at least *size* number of items.
    ///
    /// FIXME: Does this correctly match the expected semantics?
    pub fn reserve(&mut self, size: usize) {
        unsafe { sys::hb_buffer_pre_allocate(self.raw, size as u32) };
    }

    /// Returns the number of elements in the buffer, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        unsafe { sys::hb_buffer_get_length(self.raw) as usize }
    }

    /// Returns `true` if the buffer contains no data.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Sets unset buffer segment properties based on buffer Unicode
    /// contents.
    ///
    /// If buffer is not empty, it must have content type
    /// `HB_BUFFER_CONTENT_TYPE_UNICODE`.
    ///
    /// If buffer script is not set (ie. is `HB_SCRIPT_INVALID`), it will
    /// be set to the Unicode script of the first character in the buffer
    /// that has a script other than `HB_SCRIPT_COMMON`,
    /// `HB_SCRIPT_INHERITED`, and `HB_SCRIPT_UNKNOWN`.
    ///
    /// Next, if buffer direction is not set (ie. is `Direction::Invalid`),
    /// it will be set to the natural horizontal direction of the buffer
    /// script as returned by `hb_script_get_horizontal_direction()`.
    ///
    /// Finally, if buffer language is not set (ie. is `HB_LANGUAGE_INVALID`),
    /// it will be set to the process's default language as returned by
    /// `hb_language_get_default()`. This may change in the future by
    /// taking buffer script into consideration when choosing a language.
    ///
    /// ```
    /// # use harfbuzz::{Buffer, Direction, sys};
    /// let mut b = Buffer::with("Hello, world!");
    /// b.guess_segment_properties();
    /// assert_eq!(b.get_direction(), Direction::LTR);
    /// assert_eq!(b.get_script(), sys::HB_SCRIPT_LATIN);
    /// ```
    ///
    /// See also:
    ///
    /// * [`get_direction`](#method.get_direction)
    /// * [`set_direction`](#method.set_direction)
    /// * [`get_script`](#method.get_script)
    /// * [`set_script`](#method.set_script)
    /// * [`get_language`](#method.get_language)
    /// * [`set_language`](#method.set_language)
    pub fn guess_segment_properties(&mut self) {
        unsafe { sys::hb_buffer_guess_segment_properties(self.raw) };
    }

    /// Set the text flow direction of the buffer.
    ///
    /// No shaping can happen without setting buffer direction, and
    /// it controls the visual direction for the output glyphs; for
    /// RTL direction the glyphs will be reversed. Many layout features
    /// depend on the proper setting of the direction, for example,
    /// reversing RTL text before shaping, then shaping with LTR direction
    /// is not the same as keeping the text in logical order and shaping
    /// with RTL direction.
    ///
    /// See also:
    ///
    /// * [`get_direction`](#method.get_direction)
    /// * [`guess_segment_properties`](#method.guess_segment_properties)
    pub fn set_direction(&mut self, direction: Direction) {
        unsafe { sys::hb_buffer_set_direction(self.raw, direction.into()) };
    }

    /// Get the text flow direction for the buffer.
    ///
    /// See also:
    ///
    /// * [`set_direction`](#method.set_direction)
    pub fn get_direction(&self) -> Direction {
        (unsafe { sys::hb_buffer_get_direction(self.raw) }).into()
    }

    /// Sets the script of buffer to *script*.
    ///
    /// Script is crucial for choosing the proper shaping behaviour
    /// for scripts that require it (e.g. Arabic) and the which
    /// OpenType features defined in the font to be applied.
    ///
    /// See also:
    ///
    /// * [`get_script`](#method.get_script)
    /// * [`guess_segment_properties`](#method.guess_segment_properties)
    pub fn set_script(&mut self, script: sys::hb_script_t) {
        unsafe { sys::hb_buffer_set_script(self.raw, script) };
    }

    /// Get the script for the buffer.
    ///
    /// See also:
    ///
    /// * [`set_script`](#method.set_script)
    pub fn get_script(&self) -> sys::hb_script_t {
        unsafe { sys::hb_buffer_get_script(self.raw) }
    }

    /// Sets the language of buffer to *language*.
    ///
    /// Languages are crucial for selecting which OpenType feature
    /// to apply to the buffer which can result in applying
    /// language-specific behaviour. Languages are orthogonal to
    /// the scripts, and though they are related, they are different
    /// concepts and should not be confused with each other.
    ///
    /// See also:
    ///
    /// * [`get_language`](#method.get_language)
    /// * [`guess_segment_properties`](#method.guess_segment_properties)
    pub fn set_language(&mut self, language: Language) {
        unsafe { sys::hb_buffer_set_language(self.raw, language.as_raw()) };
    }

    /// Get the language for the buffer.
    ///
    /// See also:
    ///
    /// * [`set_language`](#method.set_language)
    pub fn get_language(&self) -> Language {
        unsafe { Language::from_raw(sys::hb_buffer_get_language(self.raw)) }
    }

    /// Get glyph informations for the buffer.
    pub fn glyph_infos(&self) -> Vec<GlyphInfo> {
        let mut infos: Vec<GlyphInfo> = Vec::new();
        let mut count = 0;
        unsafe {
            let ptr = sys::hb_buffer_get_glyph_infos(self.raw, &mut count);
            infos.reserve(count as usize);
            std::ptr::copy_nonoverlapping(
                ptr,
                infos.as_mut_ptr() as *mut sys::hb_glyph_info_t,
                count as usize,
            );
            infos.set_len(count as usize);
        }
        infos
    }

    /// Get glyph positions for the buffer.
    pub fn glyph_positions(&self) -> Vec<GlyphPosition> {
        let mut positions: Vec<GlyphPosition> = Vec::new();
        let mut count = 0;
        unsafe {
            let ptr = sys::hb_buffer_get_glyph_positions(self.raw, &mut count);
            positions.reserve(count as usize);
            std::ptr::copy_nonoverlapping(
                ptr,
                positions.as_mut_ptr() as *mut sys::hb_glyph_position_t,
                count as usize,
            );
            positions.set_len(count as usize);
        }
        positions
    }

    // /// Whether buffer has glyph position data
    // pub fn has_positions(&self) -> bool {
    //     unsafe { sys::hb_buffer_has_positions(self.raw) != 0 }
    // }

    /// The buffer invisible [hb_codepoint_t]
    pub fn invisible_glyph(&self) -> Codepoint {
        unsafe { sys::hb_buffer_get_invisible_glyph(self.raw) }
    }

    /// Sets the hb_codepoint_t that replaces invisible characters in the shaping result.
    /// If set to zero (default), the glyph for the U+0020 SPACE character is used.
    /// Otherwise, this value is used verbatim.
    pub fn set_invisible_glyph(&mut self, invisible: Codepoint) {
        unsafe { sys::hb_buffer_set_invisible_glyph(self.raw, invisible) };
    }

    /// Sets buffer flags to flags
    pub fn set_flags(&mut self, flags: BufferFlags) {
        unsafe { sys::hb_buffer_set_flags(self.raw, flags.bits()) };
    }

    /* Since: 3.1.0
    pub fn not_found_glyph(&self) -> Codepoint {
        unsafe { sys::hb_buffer_get_not_found_glyph(self.raw) }
    }

    pub fn set_not_found_glyph(&mut self, not_found: Codepoint) {
        unsafe { sys::hb_buffer_set_not_found_glyph(self.raw, not_found) };
    }
    */

    pub fn replacement_codepoint(&self) -> Codepoint {
        unsafe { sys::hb_buffer_get_replacement_codepoint(self.raw) }
    }

    pub fn set_replacement_codepoint(&mut self, replacement: Codepoint) {
        unsafe { sys::hb_buffer_set_replacement_codepoint(self.raw, replacement) };
    }

    pub fn normalize_glyphs(&mut self) {
        unsafe { sys::hb_buffer_normalize_glyphs(self.raw) };
    }

    pub fn reverse(&mut self) {
        unsafe { sys::hb_buffer_reverse(self.raw) };
    }

    pub fn reverse_range(&mut self, start: u32, end: u32) {
        unsafe { sys::hb_buffer_reverse_range(self.raw, start, end) };
    }

    pub fn reverse_clusters(&mut self) {
        unsafe { sys::hb_buffer_reverse_clusters(self.raw) };
    }
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Buffer")
            .field("direction", &self.get_direction())
            .field("script", &self.get_script())
            .field("language", &self.get_language())
            .finish()
    }
}

impl Default for Buffer {
    /// Create a new, empty buffer.
    fn default() -> Self {
        Buffer {
            raw: unsafe { sys::hb_buffer_create() },
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { sys::hb_buffer_destroy(self.raw) }
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct BufferFlags: u32 {
        const DEFAULT = sys::HB_BUFFER_FLAG_DEFAULT;
        const BOT = sys::HB_BUFFER_FLAG_BOT;
        const EOT = sys::HB_BUFFER_FLAG_EOT;
        const PRESERVE_DEFAULT_IGNORABLES = sys::HB_BUFFER_FLAG_PRESERVE_DEFAULT_IGNORABLES;
        const REMOVE_DEFAULT_IGNORABLES = sys::HB_BUFFER_FLAG_REMOVE_DEFAULT_IGNORABLES;
        const DO_NOT_INSERT_DOTTED_CIRCLE = sys::HB_BUFFER_FLAG_DO_NOT_INSERT_DOTTED_CIRCLE;
        // const VERIFY = sys::HB_BUFFER_FLAG_VERIFY;
        // const PRODUCE_UNSAFE_TO_CONCAT = sys::HB_BUFFER_FLAG_PRODUCE_UNSAFE_TO_CONCAT;
        // const DEFINED = sys::HB_BUFFER_FLAG_DEFINED;
    }
}

impl Default for BufferFlags {
    fn default() -> BufferFlags {
        BufferFlags::DEFAULT
    }
}
