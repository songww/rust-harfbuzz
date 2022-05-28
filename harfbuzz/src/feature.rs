use std::str::FromStr;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Feature(sys::hb_feature_t);

impl Feature {
    pub fn set_value(&mut self, value: u32) {
        self.0.value = value;
    }
    pub fn set_start(&mut self, start: u32) {
        self.0.start = start;
    }
    pub fn set_end(&mut self, end: u32) {
        self.0.end = end;
    }
}

#[derive(Debug, Clone)]
pub struct InvalidFeature(String);

impl FromStr for Feature {
    type Err = InvalidFeature;
    fn from_str(s: &str) -> Result<Feature, Self::Err> {
        let mut feature = sys::hb_feature_t {
            tag: 0,
            value: 0,
            start: 0,
            end: 0,
        };
        let ret = unsafe {
            sys::hb_feature_from_string(s.as_ptr() as *const i8, s.len() as i32, &mut feature)
        };
        if ret != 0 {
            Ok(Feature(feature))
        } else {
            Err(InvalidFeature(s.to_string()))
        }
    }
}

impl Into<String> for Feature {
    fn into(mut self) -> String {
        let mut s = String::with_capacity(128);
        unsafe {
            sys::hb_feature_to_string(&mut self.0, s.as_mut_ptr() as *mut i8, s.capacity() as u32)
        };
        s
    }
}
