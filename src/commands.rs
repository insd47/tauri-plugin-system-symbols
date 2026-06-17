use tauri::{command, State};

use crate::{
    cache::SymbolCache,
    models::{SvgSymbol, SymbolRequest},
};

#[command]
pub(crate) async fn get_fluent_icons(
    cache: State<'_, SymbolCache>,
    icons: Vec<String>,
) -> crate::Result<Vec<SvgSymbol>> {
    icons
        .into_iter()
        .map(|icon| resolve_with_cache(&cache, SymbolRequest::Fluent(icon)))
        .collect()
}

#[command]
pub(crate) async fn get_sf_symbols(
    cache: State<'_, SymbolCache>,
    symbols: Vec<String>,
) -> crate::Result<Vec<SvgSymbol>> {
    symbols
        .into_iter()
        .map(|symbol| resolve_with_cache(&cache, SymbolRequest::Sf(symbol)))
        .collect()
}

fn resolve_with_cache(cache: &SymbolCache, request: SymbolRequest) -> crate::Result<SvgSymbol> {
    if let Some(symbol) = cache.get(&request) {
        return Ok(symbol);
    }

    let symbol = crate::platform::resolve(&request)?;
    cache.insert(request, symbol.clone());
    Ok(symbol)
}
