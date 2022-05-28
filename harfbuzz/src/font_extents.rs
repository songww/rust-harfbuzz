#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FontExtents(sys::hb_font_extents_t);

impl FontExtents {
    pub fn ascender(&self) -> i32 {
        self.0.ascender
    }

    pub fn descender(&self) -> i32 {
        self.0.descender
    }

    pub fn line_gap(&self) -> i32 {
        self.0.line_gap
    }
}

impl From<sys::hb_font_extents_t> for FontExtents {
    fn from(extents: sys::hb_font_extents_t) -> Self {
        FontExtents(extents)
    }
}

impl From<FontExtents> for sys::hb_font_extents_t {
    fn from(extents: FontExtents) -> Self {
        extents.0
    }
}
