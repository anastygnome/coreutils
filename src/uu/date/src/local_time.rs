// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#[cfg(all(
    unix,
    not(target_env = "ohos"),
    any(
        target_os = "android",
        target_os = "cygwin",
        target_os = "dragonfly",
        target_os = "emscripten",
        target_os = "fuchsia",
        target_os = "freebsd",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "ios",
        target_os = "l4re",
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "nto",
        target_os = "nuttx",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "watchos"
    )
))]
mod libc_local_time {
    use super::super::{Offset, TimeZone, Timestamp, Zoned};
    use std::{ffi::CStr, mem::MaybeUninit};

    unsafe extern "C" {
        fn tzset();
    }

    pub(super) fn zoned(timestamp: Timestamp) -> Zoned {
        try_zoned(timestamp).unwrap_or_else(|| timestamp.to_zoned(super::super::system_zone()))
    }

    /// Captures libc's local offset and abbreviation for this instant.
    /// The resulting timezone has no transitions.
    fn try_zoned(timestamp: Timestamp) -> Option<Zoned> {
        let seconds = libc::time_t::try_from(timestamp.as_second()).ok()?;
        let mut raw = MaybeUninit::<libc::tm>::uninit();

        // The application must not concurrently mutate the environment.
        unsafe { tzset() };
        let result = unsafe { libc::localtime_r(std::ptr::addr_of!(seconds), raw.as_mut_ptr()) };

        if result.is_null() {
            return None;
        }

        // SAFETY: Successful localtime_r initialized `tm`.
        let tm = unsafe { raw.assume_init() };
        let offset = Offset::from_seconds(i32::try_from(tm.tm_gmtoff).ok()?).ok()?;

        if tm.tm_zone.is_null() {
            return None;
        }

        // SAFETY: libc supplies a NUL-terminated string. TZ must remain
        // unchanged while the string is accessed.
        let abbreviation = unsafe { CStr::from_ptr(tm.tm_zone) }.to_str().ok()?;

        let zone = named_fixed_zone(abbreviation, offset)?;
        Some(timestamp.to_zoned(zone))
    }

    fn named_fixed_zone(abbreviation: &str, offset: Offset) -> Option<TimeZone> {
        // POSIX abbreviations require at least three characters.
        if abbreviation.len() < 3 {
            return None;
        }

        let plain = abbreviation.bytes().all(|b| b.is_ascii_alphabetic());

        // Quoted POSIX abbreviations additionally allow digits and +/-.
        if !plain
            && !abbreviation
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'-'))
        {
            return None;
        }

        let (open, close) = if plain { ("", "") } else { ("<", ">") };

        // POSIX offsets have the opposite sign to offsets east of UTC.
        let seconds = -i64::from(offset.seconds());
        let sign = if seconds < 0 { "-" } else { "" };
        let magnitude = seconds.unsigned_abs();

        let specification = format!(
            "{open}{abbreviation}{close}{sign}{}:{:02}:{:02}",
            magnitude / 3_600,
            magnitude % 3_600 / 60,
            magnitude % 60,
        );

        TimeZone::posix(&specification).ok()
    }
}

#[cfg(all(
    unix,
    not(target_env = "ohos"),
    any(
        target_os = "android",
        target_os = "cygwin",
        target_os = "dragonfly",
        target_os = "emscripten",
        target_os = "fuchsia",
        target_os = "freebsd",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "ios",
        target_os = "l4re",
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "nto",
        target_os = "nuttx",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "watchos"
    )
))]
pub(super) fn zoned(timestamp: super::Timestamp) -> super::Zoned {
    libc_local_time::zoned(timestamp)
}

#[cfg(not(all(
    unix,
    not(target_env = "ohos"),
    any(
        target_os = "android",
        target_os = "cygwin",
        target_os = "dragonfly",
        target_os = "emscripten",
        target_os = "fuchsia",
        target_os = "freebsd",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "ios",
        target_os = "l4re",
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "nto",
        target_os = "nuttx",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "watchos"
    )
)))]
pub(super) fn zoned(timestamp: super::Timestamp) -> super::Zoned {
    timestamp.to_zoned(super::system_zone())
}
