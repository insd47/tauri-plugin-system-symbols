mod outline;
mod path;

use dispatch2::run_on_main;
use objc2::rc::autoreleasepool;

use crate::models::Path;

pub(crate) fn resolve(symbol: &str, size: f32) -> crate::Result<Vec<Path>> {
    let symbol = symbol.to_string();
    run_on_main(move |_| autoreleasepool(|_| get_sf_symbol(&symbol, size)))
}

fn get_sf_symbol(symbol: &str, size: f32) -> crate::Result<Vec<Path>> {
    let outline = outline::extract(symbol, size as f64)?;
    let d = path::to_path(&outline, size as f64)?;

    Ok(vec![Path {
        d,
        fill_rule: path::fill_rule(&outline),
        opacity: None,
    }])
}
