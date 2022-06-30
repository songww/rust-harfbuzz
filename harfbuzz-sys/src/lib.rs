#![cfg_attr(dox, feature(doc_cfg))]

#[cfg(target_vendor = "apple")]
pub mod coretext;

#[cfg(any(target_os = "android", all(unix, not(target_vendor = "apple"))))]
extern "C" {
    pub fn hb_ft_face_create_referenced(face: freetype_sys::FT_Face) -> *mut hb_face_t;
    pub fn hb_ft_font_create_referenced(face: freetype_sys::FT_Face) -> *mut hb_font_t;
    pub fn hb_ft_font_changed(font: *mut hb_font_t);
    pub fn hb_ft_font_lock_face(font: *mut hb_font_t) -> freetype_sys::FT_Face;
    pub fn hb_ft_font_unlock_face(font: *mut hb_font_t);
    pub fn hb_ft_font_set_funcs(font: *mut hb_font_t);
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(clippy::unreadable_literal)]
mod bindings {
    #[cfg(feature = "bindgen")]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    #[cfg(not(feature = "bindgen"))]
    include!("bindings.rs");
}

pub use bindings::*;

pub const fn hb_tag(c1: u8, c2: u8, c3: u8, c4: u8) -> hb_tag_t {
    c1 as u32 & 0xFF << 24 | c2 as u32 & 0xFF << 16 | c3 as u32 & 0xFF << 8 | c4 as u32 & 0xFF
}
