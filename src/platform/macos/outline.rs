use std::{ptr, ptr::NonNull};

use objc2_core_foundation::{CFRange, CFRetained, CFString};
use objc2_core_graphics::{CGGlyph, CGPath};
use objc2_core_text::CTFont;

use crate::{Error, Result};

const FAMILIES: [&str; 4] = ["SF Pro", "SF Compact", "Apple Symbols", "Helvetica"];

pub(super) fn extract(symbol: &str, size: f64) -> Result<CFRetained<CGPath>> {
    let characters: Vec<u16> = symbol.encode_utf16().collect();
    if symbol.chars().count() != 1 {
        return Err(Error::Symbol(
            "macOS SF Symbols must be passed as a copied symbol character".into(),
        ));
    }

    let text = CFString::from_str(symbol);
    let range = CFRange::new(0, characters.len() as isize);

    for family in FAMILIES {
        let name = CFString::from_str(family);
        let base = unsafe { CTFont::with_name(&name, size, ptr::null()) };
        let font = unsafe { base.for_string(&text, range) };
        if let Some(path) = path(&font, &characters) {
            return Ok(path);
        }

        if let Some(path) = path(&base, &characters) {
            return Ok(path);
        }
    }

    Err(Error::Symbol(format!(
        "SF Symbol character `{symbol}` was not found in available fonts"
    )))
}

fn path(font: &CTFont, characters: &[u16]) -> Option<CFRetained<CGPath>> {
    let glyph = glyph(font, characters)?;
    unsafe { font.path_for_glyph(glyph, ptr::null()) }
}

fn glyph(font: &CTFont, characters: &[u16]) -> Option<CGGlyph> {
    let mut glyphs = vec![0; characters.len()];
    unsafe {
        font.glyphs_for_characters(
            NonNull::new(characters.as_ptr() as *mut u16)?,
            NonNull::new(glyphs.as_mut_ptr())?,
            characters.len() as isize,
        );
    }

    glyphs.into_iter().find(|glyph| *glyph != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_symbol_names() {
        assert!(extract("square.and.arrow.up", 16.0).is_err());
    }
}
