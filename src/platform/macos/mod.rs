mod outline;
mod path;

use crate::models::Path;

pub(crate) fn resolve(symbol: &str, size: f32) -> crate::Result<Vec<Path>> {
    let outline = outline::extract(symbol, size as f64)?;
    let d = path::to_path(&outline, size as f64)?;

    Ok(vec![Path {
        d,
        fill_rule: None,
        opacity: None,
    }])
}
