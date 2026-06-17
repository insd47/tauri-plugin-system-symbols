use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use crate::models::{SvgSymbol, SymbolRequest};

#[derive(Default)]
pub(crate) struct SymbolCache {
    symbols: Mutex<HashMap<SymbolRequest, SvgSymbol>>,
}

impl SymbolCache {
    pub(crate) fn get(&self, request: &SymbolRequest) -> Option<SvgSymbol> {
        self.symbols().get(request).cloned()
    }

    pub(crate) fn insert(&self, request: SymbolRequest, symbol: SvgSymbol) {
        self.symbols().insert(request, symbol);
    }

    fn symbols(&self) -> MutexGuard<'_, HashMap<SymbolRequest, SvgSymbol>> {
        self.symbols
            .lock()
            .unwrap_or_else(|error| error.into_inner())
    }
}
