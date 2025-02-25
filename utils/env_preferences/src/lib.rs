// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # env_preferences
//!
//! `env_preferences` is a crate to retrieve system locale and preferences for
//! Apple, Linux & Windows systems
//!
//! It currently fetches locales for the operating system
//! currently in `String` format.
//!
//! In the current setup, it is not ensured that the locale retrieved will be
//! converted to [`ICU4X Locale`](https://crates.io/crates/icu_locale)
//!
//! It also retrieves preferences for [`Calendar`](https://crates.io/crates/icu_calendar)
//! & [`TimeZone`](https://crates.io/crates/icu_time)

mod error;

#[cfg(target_os = "macos")]
mod apple;
#[cfg(target_os = "linux")]
mod posix;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!(
    "Unsupported target OS. Supported operating systems are Apple, Linux & Windows as of now"
);

#[cfg(target_os = "windows")]
mod system_locale {
    use crate::windows;

    pub type SystemLocale<'src> = windows::WindowsLocale<'src>;
    /// There are no parsing errors on this platform, so just alias to the broader [`LocaleError`] enum
    pub type SystemLocaleError = crate::LocaleError;
    pub use windows::get_locales;
}

#[cfg(target_os = "linux")]
mod system_locale {
    use crate::posix;

    pub type SystemLocale<'src> = posix::PosixLocale<'src>;
    pub type SystemLocaleError = posix::PosixParseError;

    pub fn get_locales() -> Result<Vec<String>, crate::RetrievalError> {
        // TODO: fix
        Ok(posix::fetch::get_locales()?.into_values().collect())
    }
}

// Re-export all relevant error types
pub use error::{LocaleError, RetrievalError};
// Re-export the system-specific locale parsing code
pub use system_locale::{SystemLocale, SystemLocaleError};

pub fn get_raw_locales() -> Result<Vec<String>, RetrievalError> {
    system_locale::get_locales()
}

pub fn get_locales_lossy() -> Result<Vec<icu_locale::Locale>, LocaleError> {
    let raw_locales = get_raw_locales()?;
    let system_locales = raw_locales
        .iter()
        .map(String::as_str)
        .map(SystemLocale::try_from_str)
        .collect::<Result<Vec<SystemLocale>, SystemLocaleError>>()?;

    system_locales
        .iter()
        .map(SystemLocale::try_convert_lossy)
        .map(|result| result.map_err(|error| LocaleError::from(error)))
        .collect()
}
