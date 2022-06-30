use std::{cell::RefCell, rc::Rc};

use kiss3d::{
    nalgebra::{self, Point3},
    resource::Mesh,
};
use rand;

use crate::body::Body;

pub fn init_sun_lighting(mesh: &Rc<RefCell<Mesh>>) {
    let mesh = mesh.borrow_mut();

    let mut uvs_gpu_vec = mesh.uvs().write().unwrap();
    let uvs = uvs_gpu_vec.data_mut().as_mut().unwrap();
    println!("len: {}", uvs.len());

    for i in 0..uvs.len() {
        uvs[i].x = 0.7 + rand::random::<f32>() * 0.3;
    }
}

// scale is applied to mesh coords.
pub fn body_lighting(body: &mut Body, mesh: &Rc<RefCell<Mesh>>, scale: f32) {
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

    for i in 0..coords.len() {
        let pos = center + coords[i].coords * scale;
        let normal = normals[i];
        let light_dir = (Point3::default() - pos).normalize();
        let diffuse = f32::max(light_dir.dot(&normal), 0.0);
        uvs[i].x = 0.1 + 0.9 * diffuse;
    }
}
