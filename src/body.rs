use kiss3d::{
    nalgebra::{Translation3, Vector3},
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
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,

    pub sphere: Option<SceneNode>,
}

pub const RENDER_SCALE: f64 = 1e-5;

impl Body {
    pub fn sun() -> Body {
        Body {
            name: String::from("sun"),
            mass: 1.9885e+30,
            radius: 0.5 * 1.39e6,
            color: Vector3::new(1.0, 1.0, 0.3),

            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),

            sphere: None,
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

            sphere: None,
        }
    }

    pub fn render_init(&mut self, window: &mut Window) {
        let mut sphere = window.add_sphere((self.radius * RENDER_SCALE) as f32);
        sphere.set_color(
            self.color.x as f32,
            self.color.y as f32,
            self.color.z as f32,
        );
        self.sphere = Some(sphere);
    }

    pub fn render_update(&mut self) {
        let sphere = self.sphere.as_mut().unwrap();

        sphere.set_local_translation(Translation3::new(
            (self.position.x * RENDER_SCALE) as f32,
            (self.position.y * RENDER_SCALE) as f32,
            (self.position.z * RENDER_SCALE) as f32,
        ));
    }
}
