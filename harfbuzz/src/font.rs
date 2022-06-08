use sys;

use crate::{Codepoint, Direction, Face, FontExtents};

pub struct Font {
    raw: *mut sys::hb_font_t,
}

impl Font {
    pub fn new(face: &mut Face) -> Font {
        unsafe {
            Font {
                raw: sys::hb_font_create(face.as_mut_ptr()),
            }
        }
    }

    pub unsafe fn from_raw(raw: *mut sys::hb_font_t) -> Self {
        Font { raw }
    }

    /// Gives up ownership and returns a raw pointer to the font.
    pub fn into_raw(self) -> *mut sys::hb_font_t {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    /// Sets ppem.
    pub fn set_ppem(&mut self, x_ppem: u32, y_ppem: u32) {
        unsafe {
            sys::hb_font_set_ppem(self.raw, x_ppem, y_ppem);
        }
    }

    pub fn set_ptem(&mut self, ptem: f32) {
        unsafe {
            sys::hb_font_set_ptem(self.raw, ptem);
        }
    }

    pub fn set_scale(&mut self, x_scale: i32, y_scale: i32) {
        unsafe {
            sys::hb_font_set_scale(self.raw, x_scale, y_scale);
        }
    }

    // pub fn set_synthetic_slant(&mut self, synthetic_slant: f32) {
    //     unsafe {
    //         sys::hb_font_set_synthetic_slant(self.raw, synthetic_slant);
    //     }
    // }

    pub fn face(&self) -> Face {
        unsafe { Face::from_raw(sys::hb_font_get_face(self.raw)) }
    }

    pub fn glyph(&self, unicode: u32, variation_selector: u32) -> Option<Codepoint> {
        let mut glyph = 0;
        let is_found =
            unsafe { sys::hb_font_get_glyph(self.raw, unicode, variation_selector, &mut glyph) };
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
                self.raw,
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
            sys::hb_font_get_ppem(self.raw, &mut x_ppem, &mut y_ppem);
        }
        (x_ppem, y_ppem)
    }

    pub fn ptem(&self) -> f32 {
        unsafe { sys::hb_font_get_ptem(self.raw) }
    }

    pub fn scale(&self) -> (i32, i32) {
        let mut x_scale = 0;
        let mut y_scale = 0;
        unsafe {
            sys::hb_font_get_scale(self.raw, &mut x_scale, &mut y_scale);
        }
        (x_scale, y_scale)
    }

    pub fn extents_for_direction(&self, direction: &Direction) -> FontExtents {
        unsafe {
            let mut extents: std::mem::MaybeUninit<sys::hb_font_extents_t> =
                std::mem::MaybeUninit::zeroed();
            sys::hb_font_get_extents_for_direction(
                self.raw,
                sys::hb_direction_t::from(*direction),
                extents.as_mut_ptr(),
            );
            extents.assume_init().into()
        }
    }

    // pub fn serial(&self) -> u32 {
    //     unsafe { sys::hb_font_get_serial(self.raw) }
    // }
    //
    // pub fn changed(&mut self) {
    //     unsafe { sys::hb_font_changed(self.raw) }
    // }

    pub fn make_immutable(&mut self) {
        unsafe { sys::hb_font_make_immutable(self.raw) }
    }

    pub fn is_immutable(&self) -> bool {
        unsafe { sys::hb_font_is_immutable(self.raw) != 0 }
    }

    pub fn empty() -> Self {
        unsafe { Self::from_raw(sys::hb_font_get_empty()) }
    }

    /// Borrows a raw pointer to the font.
    pub fn as_ptr(&self) -> *const sys::hb_font_t {
        self.raw
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::hb_font_t {
        self.raw
    }

    user_data_methods! {
        sys::hb_font_get_user_data,
        sys::hb_font_set_user_data,
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            sys::hb_font_destroy(self.raw);
        }
    }
}

impl Clone for Font {
    fn clone(&self) -> Self {
        unsafe {
            Font {
                raw: sys::hb_font_reference(self.raw),
            }
        }
    }
}
