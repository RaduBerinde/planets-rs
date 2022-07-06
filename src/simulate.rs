use super::body::BodyProperties;
use chrono::{DateTime, TimeZone, Utc};
use kiss3d::nalgebra::{Point3, Vector3};

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
    pub fn simple() -> Snapshot {
        Snapshot {
            timestamp: Utc.ymd(1900, 1, 1).and_hms(0, 0, 0),
            earth_position: Point3::new(EARTH_APHELION, 0.0, 0.0),
            earth_velocity: Vector3::new(0.0, 0.0, 0.0),
            moon_position: Point3::new(EARTH_APHELION - 372000.0, 0.0, 0.0),
            moon_velocity: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn advance(self: &Snapshot, new_timestamp: DateTime<Utc>) -> Snapshot {
        let mut s = *self;
        loop {
            s = step(&s, DEFAULT_STEP);
            if s.timestamp >= new_timestamp {
                return s;
            }
        }
    }
}

const DEFAULT_STEP: f64 = 1.0; // seconds

fn step(s: &Snapshot, dt: f64) -> Snapshot {
    // We use velocity Verlet integration:
    //  x(t+dt) = x(t) + v(t)dt + a(t)dt^2/2
    //  v(t+dt) = v(t) + ((a(t) + a(t+dt))dt/2

    let (earth_acc, moon_acc) = gacc_earth_and_moon(&s.earth_position, &s.moon_position);

    let new_earth_pos = s.earth_position + s.earth_velocity * dt + 0.5 * earth_acc * dt * dt;
    let new_moon_pos = s.moon_position + s.moon_velocity * dt + 0.5 * moon_acc * dt * dt;

    let (new_earth_acc, new_moon_acc) = gacc_earth_and_moon(&new_earth_pos, &new_moon_pos);
    let new_earth_vel = s.earth_velocity + 0.5 * (earth_acc + new_earth_acc) * dt;
    let new_moon_vel = s.moon_velocity + 0.5 * (moon_acc + new_moon_acc) * dt;

    Snapshot {
        timestamp: s.timestamp + chrono::Duration::nanoseconds((dt * 1_000_000_000.0) as i64),
        earth_position: new_earth_pos,
        earth_velocity: new_earth_vel,
        moon_position: new_moon_pos,
        moon_velocity: new_moon_vel,
    }
}

fn gacc_earth_and_moon(
    earth_position: &Point3<f64>,
    moon_position: &Point3<f64>,
) -> (Vector3<f64>, Vector3<f64>) {
    let sun_pos = Point3::<f64>::new(0.0, 0.0, 0.0);
    let earth_acc = gacc(earth_position, &sun_pos, BodyProperties::SUN.mass)
        + gacc(earth_position, moon_position, BodyProperties::MOON.mass);

    let moon_acc = gacc(moon_position, &sun_pos, BodyProperties::SUN.mass)
        + gacc(moon_position, earth_position, BodyProperties::EARTH.mass);

    (earth_acc, moon_acc)
}

const G: f64 = 6.67430e-11; // N*m^2/kg^2

// Returns the acceleration vector due to gravity as a vector (with m/s^2 components).
fn gacc(pos: &Point3<f64>, other_pos: &Point3<f64>, other_mass: f64) -> Vector3<f64> {
    let vec = other_pos - pos;
    let amount = G * other_mass / vec.norm_squared();
    return vec.normalize() * amount;
}
