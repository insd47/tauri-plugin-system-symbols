mod font;
mod outline;
mod path;

use crate::{
    models::{SvgSymbol, SymbolRequest},
    Error,
};

pub(crate) fn resolve(request: &SymbolRequest) -> crate::Result<SvgSymbol> {
    let codepoint = parse_codepoint(&request.symbol)?;
    let (family, face, glyph_index) = font::resolve(&request.family, codepoint)?;
    let segments = outline::extract(&face, glyph_index)?;
    let path = path::to_path(&segments);

    Ok(SvgSymbol {
        family,
        symbol: request.symbol.clone(),
        path,
        view_box: format!("0 0 {0} {0}", path::VIEW),
    })
}

fn parse_codepoint(symbol: &str) -> crate::Result<u32> {
    let symbol = symbol.trim();
    if let Some(character) = single_character(symbol) {
        return Ok(character as u32);
    }

    let hex = symbol
        .strip_prefix("U+")
        .or_else(|| symbol.strip_prefix("u+"))
        .or_else(|| symbol.strip_prefix("0x"))
        .or_else(|| symbol.strip_prefix("0X"))
        .unwrap_or(symbol);

    u32::from_str_radix(hex, 16)
        .map_err(|_| Error::InvalidRequest(format!("invalid Windows symbol codepoint `{symbol}`")))
}

fn single_character(symbol: &str) -> Option<char> {
    let mut chars = symbol.chars();
    let character = chars.next()?;
    chars.next().is_none().then_some(character)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_codepoints() {
        assert_eq!(parse_codepoint("\u{E921}").unwrap(), 0xE921);
        assert_eq!(parse_codepoint("U+E921").unwrap(), 0xE921);
        assert_eq!(parse_codepoint("0xE921").unwrap(), 0xE921);
        assert_eq!(parse_codepoint("E921").unwrap(), 0xE921);
    }
}
