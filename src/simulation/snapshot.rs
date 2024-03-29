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
    pub fn solar_eclipse_aug_2017() -> Snapshot {
        // Data from https://ssd.jpl.nasa.gov/horizons/app.html
        //   - Vector table
        //   - Target body: Earth/Luna
        //   - Coordinate center: @sun
        //   - Time: 2017-09-21 15:46:48 TDB
        //
        // Earth:
        //   X = 1.290745457486534E+08 Y =-7.899200932997707E+07 Z = 2.689484561856836E+03
        //   VX= 1.507209745469294E+01 VY= 2.530788781266470E+01 VZ=-2.302676624889699E-03
        // Moon:
        //   X = 1.287626991572680E+08 Y =-7.878974778529878E+07 Z = 4.510853649180382E+03
        //   VX= 1.446304692640349E+01 VY= 2.444380218816157E+01 VZ= 9.547778835418086E-02
        #[allow(clippy::excessive_precision)]
        Snapshot {
            timestamp: Utc.ymd(2017, 8, 21).and_hms(15, 46, 48),
            earth_position: Point3::new(
                1.290745457486534E+08,
                -7.899200932997707E+07,
                2.689484561856836E+03,
            ),
            earth_velocity: Vector3::new(
                1.507209745469294E+01,
                2.530788781266470E+01,
                -2.302676624889699E-03,
            ),
            moon_position: Point3::new(
                1.287626991572680E+08,
                -7.878974778529878E+07,
                4.510853649180382E+03,
            ),
            moon_velocity: Vector3::new(
                1.446304692640349E+01,
                2.444380218816157E+01,
                9.547778835418086E-02,
            ),
        }
    }

    pub fn lunar_eclipse_may_2022() -> Snapshot {
        // Data obtained as above.
        // Earth:
        //   X =-8.698598672690395E+07 Y =-1.237132786667059E+08 Z = 6.674273178458214E+03
        //   VX= 2.387276880809248E+01 VY=-1.723966486848198E+01 VZ= 1.953072531164501E-03
        // Moon:
        //   X =-8.720249905686758E+07 Y =-1.240039134908936E+08 Z = 6.046639699906111E+03
        //   VX= 2.475907992617419E+01 VY=-1.786132315008197E+01 VZ=-9.810927783777057E-02
        #[allow(clippy::excessive_precision)]
        Snapshot {
            timestamp: Utc.ymd(2022, 5, 16).and_hms(1, 32, 7),
            earth_position: Point3::new(
                -8.698598672690395E+07,
                -1.237132786667059E+08,
                6.674273178458214E+03,
            ),
            earth_velocity: Vector3::new(
                2.387276880809248E+01,
                -1.723966486848198E+01,
                1.953072531164501E-03,
            ),
            moon_position: Point3::new(
                -8.720249905686758E+07,
                -1.240039134908936E+08,
                6.046639699906111E+03,
            ),
            moon_velocity: Vector3::new(
                2.475907992617419E+01,
                -1.786132315008197E+01,
                -9.810927783777057E-02,
            ),
        }
    }

    #[allow(dead_code)]
    pub fn test_no_moon_inclination() -> Snapshot {
        Snapshot {
            timestamp: Utc.ymd(2000, 1, 1).and_hms(0, 0, 0),
            earth_position: Point3::new(EARTH_APHELION, 0.0, 0.0),
            earth_velocity: Vector3::new(0.0, 29.3, 0.0),
            moon_position: Point3::new(EARTH_APHELION - 372_000.0, 0.0, 0.0),
            moon_velocity: Vector3::new(0.0, 29.3 - 1.022, 0.0),
        }
    }

    #[allow(dead_code)]
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
