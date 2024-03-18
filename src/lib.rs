#![doc=include_str!("../README.md")]

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Theme {
    /// Standard appearance (possibly light), or themes are not supported
    Default,

    /// The theme has a "Dark" appearance (light text on dark background)
    Dark,
}

/// Reads the current system-wide default setting for application theme.
///
/// Returns `None` if the platform is not supported.
pub fn global_default_theme() -> Option<Theme> {
    #[cfg(target_vendor = "apple")]
    return crate::apple::get();

    #[cfg(not(target_vendor = "apple"))]
    return None;
}

#[cfg(target_vendor = "apple")]
mod apple {
    use crate::Theme;
    use core_foundation_sys::preferences::kCFPreferencesAnyApplication;
    use core::ptr;
    use core_foundation_sys::base::CFRelease;
    use core_foundation_sys::base::kCFAllocatorNull;
    use core_foundation_sys::preferences::CFPreferencesCopyAppValue;
    use core_foundation_sys::string::CFStringCreateWithBytesNoCopy;
    use core_foundation_sys::string::CFStringHasPrefix;
    use core_foundation_sys::string::CFStringRef;
    use core_foundation_sys::string::kCFStringEncodingUTF8;

    fn static_cf_string(string: &'static str) -> CFStringRef {
        unsafe {
            CFStringCreateWithBytesNoCopy(
                ptr::null_mut(),
                string.as_ptr(),
                string.len() as _,
                kCFStringEncodingUTF8,
                false as _,
                kCFAllocatorNull,
            )
        }
    }

    pub(crate) fn get() -> Option<Theme> {
        #[cfg(target_vendor = "apple")]
        unsafe {
            let interface_style = static_cf_string("AppleInterfaceStyle");
            if interface_style.is_null() {
                return None;
            }
            let dark = static_cf_string("Dark");

            let value = CFPreferencesCopyAppValue(interface_style, kCFPreferencesAnyApplication);
            let is_dark = !value.is_null() && !dark.is_null() && 0 != CFStringHasPrefix(value.cast(), dark);

            CFRelease(dark.cast());
            CFRelease(interface_style.cast());

            Some(if is_dark { Theme::Dark } else { Theme::Default })
        }
    }
}

#[test]
fn test() {
    #[cfg(target_vendor = "apple")]
    assert!(global_default_theme().is_some());

    global_default_theme();
}
