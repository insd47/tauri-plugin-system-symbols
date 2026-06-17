use objc2::{
    class, msg_send,
    rc::Retained,
    runtime::{AnyObject, NSObjectProtocol},
    sel,
};
use objc2_app_kit::{NSBezierPath, NSImage, NSImageRep};
use objc2_foundation::NSString;

use crate::Error;

pub(super) fn extract(symbol: &str, size: f64) -> crate::Result<Retained<NSBezierPath>> {
    let name = NSString::from_str(symbol);
    let image = NSImage::imageWithSystemSymbolName_accessibilityDescription(&name, None)
        .ok_or_else(|| Error::Symbol(format!("SF Symbol `{symbol}` was not found")))?;
    let image = configure(&image, size).unwrap_or(image);
    let representations = image.representations();
    let representation = representations.firstObject().ok_or_else(|| {
        Error::Symbol(format!("SF Symbol `{symbol}` has no image representation"))
    })?;

    path(&representation, symbol)
}

fn configure(image: &NSImage, size: f64) -> Option<Retained<NSImage>> {
    let configuration: Option<Retained<AnyObject>> = unsafe {
        msg_send![
            class!(NSImageSymbolConfiguration),
            configurationWithPointSize: size,
            weight: 0.0f64
        ]
    };
    let configuration = configuration?;

    unsafe { msg_send![image, imageWithSymbolConfiguration: &*configuration] }
}

fn path(representation: &NSImageRep, symbol: &str) -> crate::Result<Retained<NSBezierPath>> {
    if !representation.respondsToSelector(sel!(outlinePath)) {
        return Err(Error::Symbol(
            "SF Symbols path extraction is unavailable on this macOS version".into(),
        ));
    }

    let path: Option<Retained<NSBezierPath>> = unsafe { msg_send![representation, outlinePath] };
    path.ok_or_else(|| Error::Symbol(format!("SF Symbol `{symbol}` has no outline path")))
}
