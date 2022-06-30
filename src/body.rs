use std::{cell::RefCell, rc::Rc};

use kiss3d::{
    nalgebra::{Point3, Translation3, Vector3},
    ncollide3d::procedural,
    resource::Mesh,
    scene::SceneNode,
    window::Window,
};

pub struct Body {
    // Constant fields.
    pub name: String,
    // Mass in kg.
    pub mass: f64,
    // Radius in km.
    pub radius: f64,
    pub color: Vector3<f32>,

    // Changing fields.
    pub position: Point3<f64>,
    pub velocity: Vector3<f64>,

    scene_node: Option<SceneNode>,
    mesh: Option<Rc<RefCell<Mesh>>>,
}

impl Body {
    pub fn new(name: String, mass: f64, radius: f64, color: Vector3<f32>) -> Body {
        Body {
            name,
            mass,
            radius,
            color,

            position: Point3::default(),
            velocity: Vector3::default(),

            scene_node: None,
            mesh: None,
        }
    }
    pub fn sun() -> Body {
        Body::new(
            String::from("sun"),
            1.9885e+30,
            696342.0,
            Vector3::new(1.0, 0.8, 0.3),
        )
    }

    pub fn earth() -> Body {
        Body::new(
            String::from("earth"),
            5.97237e+24,
            6378.137, // equatorial
            Vector3::new(0.1, 0.5, 1.0),
        )
    }

    pub fn moon() -> Body {
        Body::new(
            String::from("moon"),
            7.34767309e+22,
            1737.5,
            Vector3::new(0.5, 0.5, 0.5),
        )
    }
}
