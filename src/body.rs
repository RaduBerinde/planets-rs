use kiss3d::nalgebra::Vector3;

pub struct Body {
    // Constant fields.
    pub name: String,
    // Mass in kg.
    pub mass: f64,
    // Radius in km.
    pub radius: f64,
    pub color: Vector3<f32>,

    // Changing fields.
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
}

impl Body {
    pub fn sun() -> Body {
        Body {
            name: String::from("sun"),
            mass: 1.9885e+30,
            radius: 0.5 * 1.39e6,
            color: Vector3::new(0.9, 0.8, 0.0),

            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn earth() -> Body {
        Body {
            name: String::from("earth"),
            mass: 5.97237e+24,
            radius: 6378.137, // equatorial
            color: Vector3::new(0.1, 0.4, 0.9),

            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
