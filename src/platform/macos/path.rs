use std::{ffi::c_void, fmt::Write, ptr::NonNull};

use objc2_core_graphics::{CGPath, CGPathElement, CGPathElementType};

use crate::Error;

pub(super) fn to_path(path: &CGPath, size: f64) -> crate::Result<String> {
    if CGPath::is_empty(Some(path)) {
        return Err(Error::Symbol("SF Symbol outline path is empty".into()));
    }

    let bounds = CGPath::bounding_box(Some(path));
    let min_x = bounds.origin.x;
    let max_y = bounds.origin.y + bounds.size.height;
    let width = bounds.size.width.max(f64::EPSILON);
    let height = bounds.size.height.max(f64::EPSILON);
    let scale = size / width.max(height);
    let offset_x = (size - width * scale) / 2.0;
    let offset_y = (size - height * scale) / 2.0;

    let mut context = Context {
        data: String::new(),
        error: None,
        min_x,
        max_y,
        scale,
        offset_x,
        offset_y,
    };
    unsafe {
        CGPath::apply(
            Some(path),
            &mut context as *mut Context as *mut c_void,
            Some(write_element),
        );
    }

    if let Some(error) = context.error {
        return Err(Error::Symbol(error));
    }

    if context.data.is_empty() {
        return Err(Error::Symbol("SF Symbol outline path is empty".into()));
    }

    Ok(context.data)
}

struct Context {
    data: String,
    error: Option<String>,
    min_x: f64,
    max_y: f64,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
}

impl Context {
    fn map_x(&self, x: f64) -> f64 {
        (x - self.min_x) * self.scale + self.offset_x
    }

    fn map_y(&self, y: f64) -> f64 {
        (self.max_y - y) * self.scale + self.offset_y
    }
}

unsafe extern "C-unwind" fn write_element(info: *mut c_void, element: NonNull<CGPathElement>) {
    let context = unsafe { &mut *(info as *mut Context) };
    if context.error.is_some() {
        return;
    }

    let element = unsafe { element.as_ref() };
    let points = element.points.as_ptr();
    let point = |index: usize| unsafe { *points.add(index) };

    if element.r#type == CGPathElementType::MoveToPoint {
        let point = point(0);
        let _ = write!(
            context.data,
            "M{:.2} {:.2}",
            context.map_x(point.x),
            context.map_y(point.y)
        );
    } else if element.r#type == CGPathElementType::AddLineToPoint {
        let point = point(0);
        let _ = write!(
            context.data,
            "L{:.2} {:.2}",
            context.map_x(point.x),
            context.map_y(point.y)
        );
    } else if element.r#type == CGPathElementType::AddQuadCurveToPoint {
        let control = point(0);
        let point = point(1);
        let _ = write!(
            context.data,
            "Q{:.2} {:.2} {:.2} {:.2}",
            context.map_x(control.x),
            context.map_y(control.y),
            context.map_x(point.x),
            context.map_y(point.y)
        );
    } else if element.r#type == CGPathElementType::AddCurveToPoint {
        let control1 = point(0);
        let control2 = point(1);
        let point = point(2);
        let _ = write!(
            context.data,
            "C{:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
            context.map_x(control1.x),
            context.map_y(control1.y),
            context.map_x(control2.x),
            context.map_y(control2.y),
            context.map_x(point.x),
            context.map_y(point.y)
        );
    } else if element.r#type == CGPathElementType::CloseSubpath {
        context.data.push('Z');
    } else {
        context.error = Some(format!(
            "unsupported SF Symbol path element `{}`",
            element.r#type.0
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    use objc2_core_graphics::CGMutablePath;

    #[test]
    fn normalizes_y_up_font_path_into_svg_coordinates() {
        let path = CGMutablePath::new();
        unsafe {
            CGMutablePath::move_to_point(Some(&path), ptr::null(), 10.0, 10.0);
            CGMutablePath::add_line_to_point(Some(&path), ptr::null(), 10.0, 20.0);
            CGMutablePath::add_line_to_point(Some(&path), ptr::null(), 20.0, 20.0);
        }

        assert_eq!(
            to_path(&path, 16.0).unwrap(),
            "M0.00 16.00L0.00 0.00L16.00 0.00"
        );
    }
}
