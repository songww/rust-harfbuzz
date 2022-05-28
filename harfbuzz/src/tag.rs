use std::str::FromStr;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Tag(sys::hb_tag_t);

impl Tag {
    pub const NONE: Tag = Tag(sys::hb_tag(0, 0, 0, 0));
    pub const MAX: Tag = Tag(sys::hb_tag(0xFF, 0xFF, 0xFF, 0xFF));

    pub const fn pack(a: u8, b: u8, c: u8, d: u8) -> Tag {
        Tag(sys::hb_tag(a, b, c, d))
    }

    pub const fn unpack(self) -> (u8, u8, u8, u8) {
        unimplemented!()
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
