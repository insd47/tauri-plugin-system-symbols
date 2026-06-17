use crate::{
    models::{SvgSymbol, SymbolRequest},
    Error,
};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub(crate) fn resolve(request: &SymbolRequest) -> crate::Result<SvgSymbol> {
    match request {
        SymbolRequest::Fluent(icon) => fluent(icon),
        SymbolRequest::Sf(symbol) => sf(symbol),
    }
}

fn fluent(icon: &str) -> crate::Result<SvgSymbol> {
    if icon.trim().is_empty() {
        return Err(Error::InvalidRequest("icon must not be empty".into()));
    }

    fluent_platform(icon)
}

#[cfg(target_os = "windows")]
fn fluent_platform(icon: &str) -> crate::Result<SvgSymbol> {
    windows::fluent(icon)
}

#[cfg(not(target_os = "windows"))]
fn fluent_platform(_icon: &str) -> crate::Result<SvgSymbol> {
    Err(Error::UnsupportedPlatform {
        system: "Fluent Icons",
        platform: std::env::consts::OS,
    })
}

fn sf(symbol: &str) -> crate::Result<SvgSymbol> {
    if symbol.trim().is_empty() {
        return Err(Error::InvalidRequest("symbol must not be empty".into()));
    }

    sf_platform(symbol)
}

#[cfg(target_os = "macos")]
fn sf_platform(symbol: &str) -> crate::Result<SvgSymbol> {
    macos::sf(symbol)
}

#[cfg(not(target_os = "macos"))]
fn sf_platform(_symbol: &str) -> crate::Result<SvgSymbol> {
    Err(Error::UnsupportedPlatform {
        system: "SF Symbols",
        platform: std::env::consts::OS,
    })
}
