use std::{cell::RefCell, rc::Rc};

use kiss3d::{
    nalgebra::{self, Point3, Vector3},
    resource::Mesh,
};

use crate::body::Body;

pub fn init_sun_lighting(mesh: &Rc<RefCell<Mesh>>) {
    let mesh = mesh.borrow_mut();

    let mut uvs_gpu_vec = mesh.uvs().write().unwrap();
    let uvs = uvs_gpu_vec.data_mut().as_mut().unwrap();

    for uv in uvs {
        uv.x = 0.7 + rand::random::<f32>() * 0.3;
    }
}

// scale is applied to mesh coords.
pub fn body_lighting(body: &Body, mesh: &Rc<RefCell<Mesh>>, scale: f32, opaque: &Body) {
    let center: Point3<f32> = nalgebra::convert(body.position);

    let mesh = mesh.borrow_mut();

    let coords_gpu_vec = mesh.coords().read().unwrap();
    let coords = coords_gpu_vec.data().as_ref().unwrap();

    let normals_gpu_vec = mesh.normals().read().unwrap();
    let normals = normals_gpu_vec.data().as_ref().unwrap();

    let mut uvs_vec = mesh.uvs().write().unwrap();
    let uvs = uvs_vec.data_mut().as_mut().unwrap();

    assert_eq!(coords.len(), uvs.len());
    assert_eq!(normals.len(), uvs.len());
    let opaque_center: Vector3<f32> = nalgebra::convert(opaque.position.coords);

    for i in 0..coords.len() {
        let pos = center + coords[i].coords * scale;
        let normal = normals[i];
        let light_dir = (Point3::default() - pos).normalize();
        let mut diffuse = f32::max(light_dir.dot(&normal), 0.0);
        if diffuse > 0.0
            && segment_intersects_sphere(
                &Vector3::default(),
                &pos.coords,
                &opaque_center,
                opaque.radius as f32,
            )
        {
            diffuse = 0.0
        }
        uvs[i].x = 0.15 + 0.85 * diffuse;
    }
}

fn segment_intersects_sphere(
    start: &Vector3<f32>,
    end: &Vector3<f32>,
    center: &Vector3<f32>,
    radius: f32,
) -> bool {
    // Instead of using the typical ray/sphere intersection formula, we
    // determine the closest point to the center on the line, and see how far it
    // is from the center. This suffices because we don't care about the
    // intersection points, just whether there is an intersection.
    // In the future, this would also allow special handling of rays which are
    // nearly tangent (to simulate Raleigh scattering).
    let seg = end - start;

    // Project the center point onto the line and normalize distance to [0, 1].
    let t = ((center - start).dot(&seg) / seg.norm_squared()).clamp(0.0, 1.0);
    let point = start + seg * t;

    (point - center).norm_squared() <= radius * radius
}
