use crate::{models::Path, Error};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub(crate) fn resolve_symbol(symbol: &str, size: f32) -> crate::Result<Vec<Path>> {
    if symbol.trim().is_empty() {
        return Err(Error::InvalidRequest("symbol must not be empty".into()));
    }

    if !size.is_finite() || size <= 0.0 {
        return Err(Error::InvalidRequest(
            "size must be a positive finite number".into(),
        ));
    }

    #[cfg(target_os = "windows")]
    return windows::resolve(symbol, size);

    #[cfg(target_os = "macos")]
    return macos::resolve(symbol, size);

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    Err(Error::UnsupportedPlatform {
        system: "System Symbols",
        platform: std::env::consts::OS,
    })
}
