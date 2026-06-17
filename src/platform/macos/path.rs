use std::fmt::Write;

use objc2_app_kit::{NSBezierPath, NSBezierPathElement, NSWindingRule};
use objc2_foundation::NSPoint;

use crate::{models::FillRule, Error};

pub(super) fn to_path(path: &NSBezierPath, size: f64) -> crate::Result<String> {
    let bounds = path.bounds();
    let min_x = bounds.origin.x;
    let max_y = bounds.origin.y + bounds.size.height;
    let width = bounds.size.width.max(f64::EPSILON);
    let height = bounds.size.height.max(f64::EPSILON);
    let scale = size / width.max(height);
    let offset_x = (size - width * scale) / 2.0;
    let offset_y = (size - height * scale) / 2.0;
    let map_x = |x: f64| (x - min_x) * scale + offset_x;
    let map_y = |y: f64| (max_y - y) * scale + offset_y;

    let mut data = String::new();
    let mut points = [NSPoint::new(0.0, 0.0); 3];
    for index in 0..path.elementCount() {
        let element = unsafe { path.elementAtIndex_associatedPoints(index, points.as_mut_ptr()) };
        match element {
            NSBezierPathElement::MoveTo => {
                let _ = write!(data, "M{:.2} {:.2}", map_x(points[0].x), map_y(points[0].y));
            }
            NSBezierPathElement::LineTo => {
                let _ = write!(data, "L{:.2} {:.2}", map_x(points[0].x), map_y(points[0].y));
            }
            NSBezierPathElement::CubicCurveTo => {
                let _ = write!(
                    data,
                    "C{:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                    map_x(points[0].x),
                    map_y(points[0].y),
                    map_x(points[1].x),
                    map_y(points[1].y),
                    map_x(points[2].x),
                    map_y(points[2].y)
                );
            }
            NSBezierPathElement::QuadraticCurveTo => {
                let _ = write!(
                    data,
                    "Q{:.2} {:.2} {:.2} {:.2}",
                    map_x(points[0].x),
                    map_y(points[0].y),
                    map_x(points[1].x),
                    map_y(points[1].y)
                );
            }
            NSBezierPathElement::ClosePath => data.push('Z'),
            element => {
                return Err(Error::Symbol(format!(
                    "unsupported SF Symbol path element `{}`",
                    element.0
                )));
            }
        }
    }

    if data.is_empty() {
        return Err(Error::Symbol("SF Symbol outline path is empty".into()));
    }

    Ok(data)
}

pub(super) fn fill_rule(path: &NSBezierPath) -> Option<FillRule> {
    match path.windingRule() {
        NSWindingRule::EvenOdd => Some(FillRule::Evenodd),
        _ => None,
    }
}
