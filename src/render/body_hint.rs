use kiss3d::nalgebra::{Point2, Vector2};
use kiss3d::{camera::Camera, nalgebra::Point3, window::Window};

use crate::body::Body;
use crate::body::Body::*;

pub fn render_body_hint(
    body: Body,
    body_render_pos: Point3<f32>,
    camera: &dyn Camera,
    window: &mut Window,
) {
    // Only show the hint if we see the object as very small.
    let dist = (body_render_pos - camera.eye()).norm();
    if dist < body.radius() * 200.0 {
        return;
    }

    let projected =
        Point3::from_homogeneous(camera.transformation() * body_render_pos.to_homogeneous())
            .unwrap();

    if projected.z > 1.0 {
        // Object behind us.
        return;
    }

    let scale = 0.5 / window.scale_factor() as f32;
    let point = Point2::new(
        projected.x * window.width() as f32 * scale,
        projected.y * window.height() as f32 * scale,
    );

    let color = body.color3();
    let mut draw_line = |angle: f32| {
        const START: f32 = 6.0;
        const END: f32 = 14.0;
        let rad = angle.to_radians();
        let axis = Vector2::new(rad.cos(), rad.sin());

        window.draw_planar_line(&(point + axis * START), &(point + axis * END), &color);
    };

    for a in [0, 90, 180, 270] {
        // Earth gets the vertical cross-hairs; Moon gets the diagonal; Sun
        // gets both.
        if body == Sun || body == Earth {
            draw_line(a as f32);
        }
        if body == Sun || body == Moon {
            draw_line((a + 45) as f32);
        }
    }
}
