use sys;

use std::rc::Rc;

use crate::{Blob, UserDataKey};

pub struct Face {
    raw: *mut sys::hb_face_t,
}

impl Face {
    pub fn new(blob: &Blob, index: u32) -> Face {
        unsafe {
            Face {
                raw: sys::hb_face_create(blob.as_raw(), index),
            }
        }
    }

    pub unsafe fn from_raw(raw: *mut sys::hb_face_t) -> Face {
        Face { raw }
    }

    pub unsafe fn into_raw(self) -> *mut sys::hb_face_t {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    pub fn index(&self) -> u32 {
        unsafe { sys::hb_face_get_index(self.raw) }
    }

    pub fn glyph_count(&self) -> usize {
        unsafe { sys::hb_face_get_glyph_count(self.raw) as usize }
    }

    pub fn upem(&self) -> u32 {
        unsafe { sys::hb_face_get_upem(self.raw) }
    }

    pub fn user_data<T>(&self, _k: &UserDataKey<T>) -> Rc<T> {
        todo!()
    }

    pub fn set_user_data<T>(&self, _k: &UserDataKey<T>, _v: Rc<T>) {
        todo!()
    }

    pub fn is_immutable(&self) -> bool {
        unsafe { sys::hb_face_is_immutable(self.raw) != 0 }
    }

    pub fn make_immutable(&mut self) {
        unsafe { sys::hb_face_make_immutable(self.raw) }
    }

    pub fn reference_blob(&self) -> Blob {
        unsafe { Blob::from_raw(sys::hb_face_reference_blob(self.raw)) }
    }

    pub fn set_glyph_count(&mut self, count: usize) {
        unsafe { sys::hb_face_set_glyph_count(self.raw, count as u32) }
    }

    pub fn set_index(&mut self, index: u32) {
        unsafe { sys::hb_face_set_index(self.raw, index) }
    }

    pub fn set_upem(&mut self, upem: u32) {
        unsafe { sys::hb_face_set_upem(self.raw, upem) }
    }

    pub fn as_ptr(&self) -> *const sys::hb_face_t {
        self.raw
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::hb_face_t {
        self.raw
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        unsafe {
            sys::hb_face_destroy(self.raw);
        }
    }
}

impl Clone for Face {
    fn clone(&self) -> Face {
        unsafe {
            Face {
                raw: sys::hb_face_reference(self.raw),
            }
        }
    }
}
