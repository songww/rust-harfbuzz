use std::str::FromStr;

#[derive(Debug, Clone)]
#[repr(transparent)]
/// Data type for tag identifiers. Tags are four byte integers, each byte representing a character.
///
/// Tags are used to identify tables,
/// design-variation axes, scripts, languages, font features,
/// and baselines with human-readable names.
pub struct Tag(sys::hb_tag_t);

impl Tag {
    /// Unset
    pub const NONE: Tag = Tag(sys::hb_tag(0, 0, 0, 0));
    /// Maximum possible unsigned hb_tag_t.
    pub const MAX: Tag = Tag(sys::hb_tag(0xFF, 0xFF, 0xFF, 0xFF));
    /// Maximum possible signed
    pub const MAX_SIGNED: Tag = Tag(sys::hb_tag(0x7F, 0xFF, 0xFF, 0xFF));

    /// Constructs an hb_tag_t from four character literals.
    ///
    /// a: 1st character of the tag
    /// b: 2nd character of the tag
    /// c: 3rd character of the tag
    /// d: 4th character of the tag
    pub const fn pack(a: u8, b: u8, c: u8, d: u8) -> Tag {
        Tag(sys::hb_tag(a, b, c, d))
    }

    /// Extracts four character literals from an hb_tag_t.
    pub const fn unpack(self) -> (u8, u8, u8, u8) {
        let tag = self.0;
        (
            (((tag) >> 24) & 0xFF) as u8,
            (((tag) >> 16) & 0xFF) as u8,
            (((tag) >> 8) & 0xFF) as u8,
            (tag & 0xFF) as u8,
        )
    }
}

#[doc(hidden)]
impl From<sys::hb_tag_t> for Tag {
    fn from(tag: sys::hb_tag_t) -> Self {
        Self(tag)
    }
}

#[doc(hidden)]
impl From<Tag> for sys::hb_tag_t {
    fn from(tag: Tag) -> Self {
        tag.0
    }
}

impl FromStr for Tag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag = unsafe { sys::hb_tag_from_string(s.as_ptr() as *const i8, s.len() as i32) };
        if tag == Self::NONE.0 {
            Err(())
        } else {
            Ok(Tag(tag))
        }
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            let mut buf = [0; 4];
            sys::hb_tag_to_string(self.0, buf.as_mut_ptr() as *mut i8);
            fmt.write_str(std::str::from_utf8_unchecked(&buf))
        }
    }
}
