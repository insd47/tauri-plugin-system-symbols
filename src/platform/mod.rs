use crate::{
    models::{SvgSymbol, SymbolRequest},
    Error,
};

#[cfg(target_os = "windows")]
mod windows;

pub(crate) fn resolve(request: &SymbolRequest) -> crate::Result<SvgSymbol> {
    if request.symbol.trim().is_empty() {
        return Err(Error::InvalidRequest("symbol must not be empty".into()));
    }

    resolve_platform(request)
}

#[cfg(target_os = "windows")]
fn resolve_platform(request: &SymbolRequest) -> crate::Result<SvgSymbol> {
    windows::resolve(request)
}

#[cfg(not(target_os = "windows"))]
fn resolve_platform(request: &SymbolRequest) -> crate::Result<SvgSymbol> {
    Err(Error::UnsupportedPlatform {
        family: request.family.clone(),
    })
}
