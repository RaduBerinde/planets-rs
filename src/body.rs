use chrono::{DateTime, TimeZone, Timelike, Utc};
use kiss3d::nalgebra::{Point3, Unit, UnitQuaternion, Vector3};

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

    pub fn radius(&self) -> f32 {
        self.props().radius as f32
    }

    pub fn radius64(&self) -> f64 {
        self.props().radius
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

const EARTH_TROPICAL_YEAR: f64 = 365.2412 * 24.0 * 3600.0;
const EARTH_TILT: f64 = 23.4;

// relative_earth_orientation calculates the rotation that needs to be applied
// to Earth so that it matches the given timestamp.
//
// It is assumed that the Sun is directly to the left (-x axis), and that the
// Earth's UTC timezone faces the sun.
pub fn relative_earth_orientation(timestamp: &DateTime<Utc>) -> UnitQuaternion<f64> {
    let (h, m, s) = (timestamp.hour(), timestamp.minute(), timestamp.second());
    let delta_seconds = (s + 60 * (m + 60 * h)) as f64;

    let rotation_angle = std::f64::consts::PI * (delta_seconds / (12.0 * 3600.0) - 1.0);

    let known_solstice = Utc.ymd(2000, 6, 21).and_hms(1, 47, 43);
    let delta = timestamp
        .signed_duration_since(known_solstice)
        .num_seconds() as f64;

    // 0 is the summer solstice and PI is the winter solstice.
    let axis_orientation =
        (delta % EARTH_TROPICAL_YEAR) / EARTH_TROPICAL_YEAR * 2.0 * std::f64::consts::PI;
    // At angle 0, we have to rotate around the y axis vector.
    let axis = Vector3::new(axis_orientation.sin(), axis_orientation.cos(), 0.0);

    UnitQuaternion::from_axis_angle(&Unit::new_normalize(axis), -EARTH_TILT.to_radians())
        * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), rotation_angle)
}

//pub fn earth_rotation_angle(&self) -> f64 {
//    let v = self.earth_position;
//    let noon_angle = f64::atan2(v.y, v.x);
//    let (h, m, s) = (
//        self.timestamp.hour(),
//        self.timestamp.minute(),
//        self.timestamp.second(),
//    );
//    let delta_seconds = (s + 60 * (m + 60 * h)) as f64;
//    noon_angle + std::f64::consts::PI * (delta_seconds / (12.0 * 3600.0) - 1.0)
//}
//
