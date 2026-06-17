use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SymbolFamily {
    Auto,
    SfSymbols,
    SegoeFluentIcons,
    SegoeMdl2Assets,
}

impl Default for SymbolFamily {
    fn default() -> Self {
        Self::Auto
    }
}

impl fmt::Display for SymbolFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Auto => "auto",
            Self::SfSymbols => "SF Symbols",
            Self::SegoeFluentIcons => "Segoe Fluent Icons",
            Self::SegoeMdl2Assets => "Segoe MDL2 Assets",
        };
        formatter.write_str(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolRequest {
    #[serde(default)]
    pub family: SymbolFamily,
    pub symbol: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgSymbol {
    pub family: SymbolFamily,
    pub symbol: String,
    pub path: String,
    pub view_box: String,
}
