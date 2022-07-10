use kiss3d::nalgebra::Point3;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Body {
    Sun,
    Earth,
    Moon,
}

impl Body {
    pub fn props(&self) -> &'static BodyProperties {
        match self {
            Body::Sun => &BodyProperties::SUN,
            Body::Earth => &BodyProperties::EARTH,
            Body::Moon => &BodyProperties::MOON,
        }
    }
}

pub struct BodyProperties {
    // Mass in kg.
    pub mass: f64,
    // Radius in km.
    pub radius: f64,
    // Color for rendering (RGB).
    pub color: (f32, f32, f32),
}

impl BodyProperties {
    pub const SUN: BodyProperties = BodyProperties {
        mass: 1.9885e+30,
        radius: 696342.0,
        color: (1.0, 0.8, 0.3),
    };

    pub const EARTH: BodyProperties = BodyProperties {
        mass: 5.97237e+24,
        radius: 6378.137, // equatorial
        color: (0.1, 0.5, 1.0),
    };

    pub const MOON: BodyProperties = BodyProperties {
        mass: 7.34767309e+22,
        radius: 1737.5,
        color: (0.7, 0.7, 0.7),
    };

    pub fn color_vec(&self) -> Point3<f32> {
        Point3::new(self.color.0, self.color.1, self.color.2)
    }
}
