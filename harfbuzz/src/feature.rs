use std::str::FromStr;

#[derive(Debug, Clone)]
#[repr(transparent)]
/// The structure that holds information about requested feature application.
/// The feature will be applied with the given value to all glyphs
/// which are in clusters between start (inclusive) and end (exclusive).
/// Setting start to HB_FEATURE_GLOBAL_START and end to HB_FEATURE_GLOBAL_END
/// specifies that the feature always applies to the entire buffer.
pub struct Feature(sys::hb_feature_t);

impl Feature {
    /// Special setting for hb_feature_t.end to apply the feature from to the end of the buffer.
    const GLOBAL_END: u32 = u32::MAX;
    /// Special setting for hb_feature_t.start to apply the feature from the start of the buffer.
    const GLOBAL_START: u32 = 0;

    /// Set the Tag of the feature
    pub fn set_tag(&mut self, tag: crate::tag::Tag) {
        self.0.tag = tag.into();
    }

    /// The value of the feature.
    /// 0 disables the feature, non-zero (usually 1) enables the feature.
    /// For features implemented as lookup type 3 (like 'salt') the value is a one based index into the alternates.
    pub fn set_value(&mut self, value: u32) {
        self.0.value = value;
    }

    /// The cluster to start applying this feature setting (inclusive).
    pub fn set_start(&mut self, start: u32) {
        self.0.start = start;
    }

    /// The cluster to end applying this feature setting (exclusive).
    pub fn set_end(&mut self, end: u32) {
        self.0.end = end;
    }
}

#[doc(hidden)]
impl From<sys::hb_feature_t> for Feature {
    fn from(feature: sys::hb_feature_t) -> Self {
        Self(feature)
    }
}

#[doc(hidden)]
impl From<Feature> for sys::hb_feature_t {
    fn from(feature: Feature) -> Self {
        feature.0
    }
}

// #[derive(Debug, Clone)]
// pub struct InvalidFeature(String);

/// Parses a string into a hb_feature_t.
///
/// The format for specifying feature strings follows.
/// All valid CSS font-feature-settings values other than 'normal' and the global values are also accepted,
/// though not documented below. CSS string escapes are not supported.
///
/// The range indices refer to the positions between Unicode characters.
/// The position before the first character is always 0.
///
/// The format is Python-esque. Here is how it all works:
/// Syntax 	Value 	Start 	End
/// Setting value:
///     kern 	1 	0 	∞ 	Turn feature on
///     +kern 	1 	0 	∞ 	Turn feature on
///     -kern 	0 	0 	∞ 	Turn feature off
///     kern=0 	0 	0 	∞ 	Turn feature off
///     kern=1 	1 	0 	∞ 	Turn feature on
///     aalt=2 	2 	0 	∞ 	Choose 2nd alternate
///     Setting index:
///     kern[] 	1 	0 	∞ 	Turn feature on
///     kern[:] 	1 	0 	∞ 	Turn feature on
///     kern[5:] 	1 	5 	∞ 	Turn feature on, partial
///     kern[:5] 	1 	0 	5 	Turn feature on, partial
///     kern[3:5] 	1 	3 	5 	Turn feature on, range
///     kern[3] 	1 	3 	3+1 	Turn feature on, single char
///     Mixing it all:
///     aalt[3:5]=2 	2 	3 	5 	Turn 2nd alternate on for range
impl FromStr for Feature {
    type Err = ();
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
            Err(())
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
