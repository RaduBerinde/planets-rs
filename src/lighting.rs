use kiss3d::nalgebra::Point3;
use rand;

use super::body::Body;

pub fn init_sun_lighting(sun: &mut Body) {
    let mesh = sun.mesh().borrow_mut();

    let mut uvs_gpu_vec = mesh.uvs().write().unwrap();
    let uvs = uvs_gpu_vec.data_mut().as_mut().unwrap();
    println!("len: {}", uvs.len());

    for i in 0..uvs.len() {
        uvs[i].x = 0.7 + rand::random::<f32>() * 0.3;
    }
}

pub fn body_lighting(body: &mut Body) {
    let center = body.render_position();

    let mesh = body.mesh().borrow_mut();

    let coords_gpu_vec = mesh.coords().read().unwrap();
    let coords = coords_gpu_vec.data().as_ref().unwrap();

    let normals_gpu_vec = mesh.coords().read().unwrap();
    let normals = normals_gpu_vec.data().as_ref().unwrap();

    let mut uvs_vec = mesh.uvs().write().unwrap();
    let uvs = uvs_vec.data_mut().as_mut().unwrap();

    assert_eq!(coords.len(), uvs.len());
    assert_eq!(normals.len(), uvs.len());

    for i in 0..coords.len() {
        let pos = center + coords[i].coords;
        let normal = normals[i];
        let light_dir = (Point3::default() - pos).normalize();
        let diffuse = f32::max(light_dir.dot(&normal.coords), 0.0);
        uvs[i].x = 0.1 + 0.9 * diffuse;
    }
}
