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
const AU: f64 = 149597870.700;

impl Snapshot {
    pub fn solar_eclipse_2017() -> Snapshot {
        // Data from https://ssd.jpl.nasa.gov/horizons/app.html
        Snapshot {
            // 2457987.157500000 = A.D. 2017-Aug-21 15:46:48.0000 TDB
            timestamp: Utc.ymd(2017, 08, 21).and_hms(15, 46, 48),
            //  X = 1.294491616887162E+08 Y =-7.818228126972641E+07 Z =-1.774548416482285E+04
            earth_position: Point3::new(1.294491616887162E+08, -7.818228126972641E+07, 0.0),
            //  VX= 1.506381775502787E+01 VY= 2.531783332291881E+01 VZ=-2.117008936265208E-03
            earth_velocity: Vector3::new(1.506381775502787E+01, 2.531783332291881E+01, 0.0),
            //  X = 1.291373150973308E+08 Y =-7.798001972504812E+07 Z =-1.592411507749930E+04
            moon_position: Point3::new(
                1.291373150973308E+08,
                -7.798001972504812E+07,
                -1.592411507749930E+04 + 1.774548416482285E+04,
            ),
            //  VX= 1.445476722673842E+01 VY= 2.445374769841568E+01 VZ= 9.566345604280535E-02
            moon_velocity: Vector3::new(
                1.445476722673842E+01,
                2.445374769841568E+01,
                9.566345604280535E-02 + 2.117008936265208E-03,
            ),
        }
    }

    pub fn test_no_moon_inclination() -> Snapshot {
        Snapshot {
            timestamp: Utc.ymd(2000, 1, 1).and_hms(0, 0, 0),
            earth_position: Point3::new(EARTH_APHELION, 0.0, 0.0),
            earth_velocity: Vector3::new(0.0, 29.3, 0.0),
            moon_position: Point3::new(EARTH_APHELION - 372_000.0, 0.0, 0.0),
            moon_velocity: Vector3::new(0.0, 29.3 - 1.022, 0.0),
        }
    }

    pub fn test_high_moon_inclination() -> Snapshot {
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
