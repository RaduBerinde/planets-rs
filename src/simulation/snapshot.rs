use chrono::{DateTime, TimeZone, Utc};
use kiss3d::nalgebra::{Point3, UnitQuaternion, Vector3};

use crate::body::relative_earth_orientation;

#[derive(Copy, Clone)]
pub struct Snapshot {
    pub timestamp: DateTime<Utc>,
    pub earth_position: Point3<f64>,
    pub earth_velocity: Vector3<f64>,
    pub moon_position: Point3<f64>,
    pub moon_velocity: Vector3<f64>,
}

const EARTH_APHELION: f64 = 152.10e6;

impl Snapshot {
    pub fn test1() -> Snapshot {
        Snapshot {
            timestamp: Utc.ymd(2000, 1, 1).and_hms(0, 0, 0),
            earth_position: Point3::new(EARTH_APHELION, 0.0, 0.0),
            earth_velocity: Vector3::new(0.0, 29.3, 0.0),
            moon_position: Point3::new(EARTH_APHELION - 372_000.0, 0.0, 0.0),
            moon_velocity: Vector3::new(0.0, 29.3 - 1.022, 0.0),
        }
    }

    pub fn test2() -> Snapshot {
        Snapshot {
            timestamp: Utc.ymd(2000, 1, 1).and_hms(0, 0, 0),
            earth_position: Point3::new(EARTH_APHELION, 0.0, 0.0),
            earth_velocity: Vector3::new(0.0, 29.3, 0.0),
            moon_position: Point3::new(EARTH_APHELION - 372_000.0, 0.0, 3_000.0),
            moon_velocity: Vector3::new(0.0, 29.3 - 1.022, 0.0),
        }
    }

    pub fn earth_orientation(&self) -> UnitQuaternion<f64> {
        let angle_around_sun = f64::atan2(self.earth_position.y, self.earth_position.x);
        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), angle_around_sun)
            * relative_earth_orientation(&self.timestamp)
    }

    pub fn moon_orientation(&self) -> UnitQuaternion<f64> {
        let earth_angle = f64::atan2(
            self.moon_position.y - self.earth_position.y,
            self.moon_position.x - self.earth_position.x,
        );
        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), earth_angle)
    }
}
