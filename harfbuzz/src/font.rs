use std::ops::Deref;
use std::ptr::NonNull;

use sys;

use crate::{Codepoint, Direction, Face, FontExtents};

#[repr(transparent)]
pub struct FontMut(NonNull<sys::hb_font_t>);

#[repr(transparent)]
pub struct Font(NonNull<sys::hb_font_t>);

impl Font {
    pub fn new(face: &mut Face) -> FontMut {
        unsafe { FontMut(NonNull::new(sys::hb_font_create(face.as_mut_ptr())).unwrap()) }
    }
}

impl FontMut {
    pub fn new(face: &mut Face) -> FontMut {
        unsafe { FontMut(NonNull::new(sys::hb_font_create(face.as_mut_ptr())).unwrap()) }
    }

    #[doc(hidden)]
    /// # SAFETY Must be valid pointer to hb_font_t
    pub unsafe fn from_raw(raw: *mut sys::hb_font_t) -> Self {
        FontMut(NonNull::new(raw).unwrap())
    }
    //
    // /// Gives up ownership and returns a raw pointer to the font.
    // pub fn into_raw(self) -> *mut sys::hb_font_t {
    //     let raw = self.as_mut_ptr();
    //     std::mem::forget(self);
    //     raw
    // }

    /// Sets ppem.
    pub fn set_ppem(&mut self, x_ppem: u32, y_ppem: u32) {
        unsafe {
            sys::hb_font_set_ppem(self.as_mut_ptr(), x_ppem, y_ppem);
        }
    }

    pub fn set_ptem(&mut self, ptem: f32) {
        unsafe {
            sys::hb_font_set_ptem(self.as_mut_ptr(), ptem);
        }
    }

    pub fn set_scale(&mut self, x_scale: i32, y_scale: i32) {
        unsafe {
            sys::hb_font_set_scale(self.as_mut_ptr(), x_scale, y_scale);
        }
    }

    // pub fn set_synthetic_slant(&mut self, synthetic_slant: f32) {
    //     unsafe {
    //         sys::hb_font_set_synthetic_slant(self.as_mut_ptr(), synthetic_slant);
    //     }
    // }

    #[inline]
    pub fn into_immutable(mut self) -> Font {
        unsafe { sys::hb_font_make_immutable(self.as_mut_ptr()) }
        let raw = self.0;
        self.0 = NonNull::dangling();
        Font(raw)
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&self) -> *mut sys::hb_font_t {
        self.0.as_ptr()
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const sys::hb_font_t {
        self.0.as_ptr()
    }

    user_data_methods! {
        sys::hb_font_get_user_data,
        sys::hb_font_set_user_data,
    }
}

impl Font {
    pub fn face(&self) -> Face {
        unsafe { Face::from_raw(sys::hb_font_get_face(self.as_mut_ptr())) }
    }

    pub fn glyph(&self, unicode: u32, variation_selector: u32) -> Option<Codepoint> {
        let mut glyph = 0;
        let is_found = unsafe {
            sys::hb_font_get_glyph(self.as_mut_ptr(), unicode, variation_selector, &mut glyph)
        };
        if is_found != 0 {
            Some(glyph)
        } else {
            None
        }
    }

    pub fn glyph_from_name(&self, name: &str) -> Option<Codepoint> {
        let mut glyph = 0;
        let is_found = unsafe {
            sys::hb_font_get_glyph_from_name(
                self.as_mut_ptr(),
                name.as_ptr() as *const i8,
                name.len() as i32,
                &mut glyph,
            )
        };
        if is_found != 0 {
            Some(glyph)
        } else {
            None
        }
    }

    pub fn ppem(&self) -> (u32, u32) {
        let mut x_ppem = 0;
        let mut y_ppem = 0;
        unsafe {
            sys::hb_font_get_ppem(self.as_mut_ptr(), &mut x_ppem, &mut y_ppem);
        }
        (x_ppem, y_ppem)
    }

    pub fn ptem(&self) -> f32 {
        unsafe { sys::hb_font_get_ptem(self.as_mut_ptr()) }
    }

    pub fn scale(&self) -> (i32, i32) {
        let mut x_scale = 0;
        let mut y_scale = 0;
        unsafe {
            sys::hb_font_get_scale(self.as_mut_ptr(), &mut x_scale, &mut y_scale);
        }
        (x_scale, y_scale)
    }

    pub fn extents_for_direction(&self, direction: &Direction) -> FontExtents {
        unsafe {
            let mut extents: std::mem::MaybeUninit<sys::hb_font_extents_t> =
                std::mem::MaybeUninit::zeroed();
            sys::hb_font_get_extents_for_direction(
                self.as_mut_ptr(),
                sys::hb_direction_t::from(*direction),
                extents.as_mut_ptr(),
            );
            extents.assume_init().into()
        }
    }

    // pub fn serial(&self) -> u32 {
    //     unsafe { sys::hb_font_get_serial(self.as_mut_ptr()) }
    // }
    //
    // pub fn changed(&mut self) {
    //     unsafe { sys::hb_font_changed(self.as_mut_ptr()) }
    // }

    pub fn is_immutable(&self) -> bool {
        unsafe { sys::hb_font_is_immutable(self.as_mut_ptr()) != 0 }
    }

    pub fn empty() -> Self {
        // # Safety: It will not be null.
        unsafe { Self(NonNull::new_unchecked(sys::hb_font_get_empty())) }
    }

    pub fn sub_font(&self) -> FontMut {
        unsafe { FontMut(NonNull::new(sys::hb_font_create_sub_font(self.as_mut_ptr())).unwrap()) }
    }

    /// Borrows a raw pointer to the font.
    pub fn as_ptr(&self) -> *const sys::hb_font_t {
        self.0.as_ptr()
    }

    fn as_mut_ptr(&self) -> *mut sys::hb_font_t {
        self.0.as_ptr()
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            sys::hb_font_destroy(self.as_mut_ptr());
        }
    }
}

impl AsRef<Font> for FontMut {
    fn as_ref(&self) -> &Font {
        self as &Font
    }
}

impl Deref for FontMut {
    type Target = Font;

    fn deref(&self) -> &Self::Target {
        // # Safety: That two pointers have same layout, and it is not null.
        unsafe { &*(self as *const Self as *const Font) }
    }
}

impl Clone for FontMut {
    fn clone(&self) -> Self {
        self.sub_font()
    }
}
