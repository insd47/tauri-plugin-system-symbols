use windows::{
    core::PCWSTR,
    Win32::Graphics::DirectWrite::{
        DWriteCreateFactory, IDWriteFactory, IDWriteFontCollection, IDWriteFontFace,
        DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL,
        DWRITE_FONT_WEIGHT_NORMAL,
    },
};
use windows_core::BOOL;

use crate::{models::SymbolFamily, Error, Result};

const FAMILIES: [(SymbolFamily, &str); 2] = [
    (SymbolFamily::SegoeFluentIcons, "Segoe Fluent Icons"),
    (SymbolFamily::SegoeMdl2Assets, "Segoe MDL2 Assets"),
];

pub(super) fn resolve(
    family: &SymbolFamily,
    codepoint: u32,
) -> Result<(SymbolFamily, IDWriteFontFace, u16)> {
    let candidates: Vec<(SymbolFamily, &str)> = match family {
        SymbolFamily::Auto => FAMILIES.to_vec(),
        SymbolFamily::SegoeFluentIcons => vec![FAMILIES[0].clone()],
        SymbolFamily::SegoeMdl2Assets => vec![FAMILIES[1].clone()],
        SymbolFamily::SfSymbols => {
            return Err(Error::UnsupportedPlatform {
                family: SymbolFamily::SfSymbols,
            });
        }
    };

    unsafe {
        let factory: IDWriteFactory =
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).map_err(win_err)?;

        let mut collection: Option<IDWriteFontCollection> = None;
        factory
            .GetSystemFontCollection(&mut collection, false)
            .map_err(win_err)?;
        let collection =
            collection.ok_or_else(|| Error::Symbol("system font collection unavailable".into()))?;

        for (family, font_name) in candidates {
            let Some(face) = create_face(&collection, font_name)? else {
                continue;
            };

            let mut index: u16 = 0;
            face.GetGlyphIndices(&codepoint, 1, &mut index)
                .map_err(win_err)?;

            if index != 0 {
                return Ok((family, face, index));
            }
        }
    }

    Err(Error::Symbol(format!(
        "no glyph found for U+{codepoint:04X}"
    )))
}

unsafe fn create_face(
    collection: &IDWriteFontCollection,
    family: &str,
) -> Result<Option<IDWriteFontFace>> {
    let name: Vec<u16> = family.encode_utf16().chain(std::iter::once(0)).collect();
    let mut index = 0u32;
    let mut exists = BOOL(0);

    collection
        .FindFamilyName(PCWSTR(name.as_ptr()), &mut index, &mut exists)
        .map_err(win_err)?;

    if !exists.as_bool() {
        return Ok(None);
    }

    let family = collection.GetFontFamily(index).map_err(win_err)?;
    let font = family
        .GetFirstMatchingFont(
            DWRITE_FONT_WEIGHT_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            DWRITE_FONT_STYLE_NORMAL,
        )
        .map_err(win_err)?;

    Ok(Some(font.CreateFontFace().map_err(win_err)?))
}

fn win_err(error: windows::core::Error) -> Error {
    Error::Symbol(error.to_string())
}
