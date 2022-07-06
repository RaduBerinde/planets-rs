use std::{
    ops::{Div, Mul},
    time::Instant,
};

use crate::control::ControlEvent;

use super::body::BodyProperties;
use chrono::{DateTime, Duration, TimeZone, Utc};
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
            earth_velocity: Vector3::new(0.0, 29.3, 0.0),
            moon_position: Point3::new(EARTH_APHELION - 372_000.0, 0.0, 0.0),
            moon_velocity: Vector3::new(0.0, 29.3 + 1.022, 0.0),
        }
    }

    pub fn advance_to(self: &Snapshot, new_timestamp: DateTime<Utc>) -> Snapshot {
        let mut s = *self;
        loop {
            s = step(&s, DEFAULT_STEP);
            if s.timestamp >= new_timestamp {
                return s;
            }
        }
    }
}

const DEFAULT_STEP: f64 = 60.0; // seconds

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

    //println!(
    //    "sun-earth acc: {}",
    //    gacc(earth_position, &sun_pos, BodyProperties::SUN.mass)
    //);

    let earth_acc = gacc(earth_position, &sun_pos, BodyProperties::SUN.mass)
        + gacc(earth_position, moon_position, BodyProperties::MOON.mass);

    let moon_acc = gacc(moon_position, &sun_pos, BodyProperties::SUN.mass)
        + gacc(moon_position, earth_position, BodyProperties::EARTH.mass);

    (earth_acc, moon_acc)
}

const G: f64 = 6.67430e-11; // N*m^2/kg^2

// Returns the acceleration vector due to gravity as a vector (with km/s^2 components).
fn gacc(pos: &Point3<f64>, other_pos: &Point3<f64>, other_mass: f64) -> Vector3<f64> {
    let vec = other_pos - pos;
    // The 1e-9 adjustment is km^2 -> m^2 conversion for the denominator
    // and m -> km conversion for the result.
    let amount = G * other_mass / vec.norm_squared() * 1e-9;
    return vec.normalize() * amount;
}

pub struct Simulation {
    pub current: Snapshot,
    // Simulated duration per elapsed second.
    pub speed: chrono::Duration,
    pub state: State,
}

pub struct StartInfo {
    instant: Instant,
    timestamp: DateTime<Utc>,
}

pub enum State {
    Stopped,
    Running(StartInfo),
}

impl Simulation {
    pub fn new(start: Snapshot) -> Self {
        Simulation {
            current: start,
            //speed: chrono::Duration::days(1),
            speed: chrono::Duration::seconds(1),
            state: State::Stopped,
        }
    }

    pub fn start(&mut self) {
        self.state = State::Running(StartInfo {
            instant: Instant::now(),
            timestamp: self.current.timestamp,
        });
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped
    }

    pub fn toggle_start(&mut self) {
        match &self.state {
            State::Running(_) => self.stop(),
            State::Stopped => self.start(),
        }
    }

    pub fn advance(&mut self) {
        match &self.state {
            State::Running(start_info) => {
                let elapsed_secs = start_info.instant.elapsed().as_secs_f64();
                let new_timestamp = start_info.timestamp
                    + chrono::Duration::seconds(
                        (self.speed.num_seconds() as f64 * elapsed_secs) as i64,
                    );
                self.current = self.current.advance_to(new_timestamp);
            }
            _ => {}
        }
    }

    pub fn adjust_speed(&mut self, new_speed: chrono::Duration) {
        match &self.state {
            State::Running(_) => {
                // We need to stop and restart because advance assumes the
                // speed is unchanged since start.
                self.stop();
                self.speed = new_speed;
                self.start();
            }

            State::Stopped => {
                self.speed = new_speed;
            }
        }
    }

    pub fn handle_event(&mut self, ev: ControlEvent) {
        match ev {
            ControlEvent::StartStop => self.toggle_start(),
            ControlEvent::Faster => self.adjust_speed(self.speed.mul(2)),
            ControlEvent::Slower => self.adjust_speed(self.speed.div(2)),
            _ => {}
        }
    }
}
