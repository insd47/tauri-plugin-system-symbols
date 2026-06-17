use tauri::{command, State};

use crate::{
    cache::SymbolCache,
    models::{SvgSymbol, SymbolRequest},
};

#[command]
pub(crate) async fn get_symbol(
    cache: State<'_, SymbolCache>,
    request: SymbolRequest,
) -> crate::Result<SvgSymbol> {
    resolve_with_cache(&cache, request)
}

#[command]
pub(crate) async fn get_symbols(
    cache: State<'_, SymbolCache>,
    requests: Vec<SymbolRequest>,
) -> crate::Result<Vec<SvgSymbol>> {
    requests
        .into_iter()
        .map(|request| resolve_with_cache(&cache, request))
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
