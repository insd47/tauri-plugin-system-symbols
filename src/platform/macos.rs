use crate::{models::SvgSymbol, Error};

pub(crate) fn sf(symbol: &str) -> crate::Result<SvgSymbol> {
    Err(Error::Symbol(format!(
        "SF Symbols extraction is not implemented yet for `{symbol}`"
    )))
}
