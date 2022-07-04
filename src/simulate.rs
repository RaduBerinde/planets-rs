use chrono::{DateTime, Utc};
use kiss3d::nalgebra::{Point3, Vector3};

struct SystemState {
    timestamp: DateTime<Utc>,
    earth_position: Point3<f64>,
    earth_velocity: Vector3<f64>,
    moon_position: Point3<f64>,
    moon_velocity: Vector3<f64>,
}
