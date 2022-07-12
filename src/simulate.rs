use std::{
    ops::{Div, Mul},
    time::Instant,
};

use super::{
    body::{relative_earth_orientation, BodyProperties},
    choice::Choice,
    control::ControlEvent,
};
use chrono::{DateTime, TimeZone, Timelike, Utc};
use kiss3d::nalgebra::{Point3, UnitQuaternion, Vector3};

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
            timestamp: Utc.ymd(2000, 1, 1).and_hms(0, 0, 0),
            earth_position: Point3::new(EARTH_APHELION, 0.0, 0.0),
            earth_velocity: Vector3::new(0.0, 29.3, 0.0),
            moon_position: Point3::new(EARTH_APHELION - 372_000.0, 0.0, 0.0),
            moon_velocity: Vector3::new(0.0, 29.3 + 1.022, 0.0),
        }
    }

    fn advance_to(self: &Snapshot, new_timestamp: DateTime<Utc>, dt: Seconds) -> Snapshot {
        assert!(dt.0 > 0.0);
        let mut s = *self;
        while s.timestamp + dt.to_duration() <= new_timestamp {
            s = step(&s, dt);
        }
        s
    }

    pub fn earth_rotation_angle(&self) -> f64 {
        let v = self.earth_position;
        let noon_angle = f64::atan2(v.y, v.x);
        let (h, m, s) = (
            self.timestamp.hour(),
            self.timestamp.minute(),
            self.timestamp.second(),
        );
        let delta_seconds = (s + 60 * (m + 60 * h)) as f64;
        noon_angle + std::f64::consts::PI * (delta_seconds / (12.0 * 3600.0) - 1.0)
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

const DEFAULT_STEP: Seconds = Seconds(60.0);
const MIN_STEPS_PER_WALL_SECOND: f64 = 100.0;
const MAX_STEPS_PER_WALL_SECOND: f64 = 10000.0;

fn step(s: &Snapshot, dt: Seconds) -> Snapshot {
    let new_timestamp = s.timestamp + dt.to_duration();
    let dt = dt.0;

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
        timestamp: new_timestamp,
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
    pub speed: Choice<chrono::Duration>,
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
        let speeds = [
            chrono::Duration::minutes(1),
            chrono::Duration::minutes(15),
            chrono::Duration::hours(1),
            chrono::Duration::hours(4),
            chrono::Duration::days(1),
            chrono::Duration::days(5),
            chrono::Duration::days(30),
            chrono::Duration::days(90),
        ];

        Simulation {
            current: start,
            speed: Choice::new_with_initial(speeds, 2),
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
                let elapsed = Seconds::from(start_info.instant.elapsed());
                assert!(elapsed.0 > 0.0);

                let simulation_speed_per_sec = Seconds::from(self.speed.get());
                let simulation_elapsed = elapsed * simulation_speed_per_sec.0;
                let new_timestamp = start_info.timestamp + simulation_elapsed.to_duration();

                let mut step = DEFAULT_STEP;
                step = step.at_least(simulation_speed_per_sec / MAX_STEPS_PER_WALL_SECOND);
                step = step.at_most(simulation_speed_per_sec / MIN_STEPS_PER_WALL_SECOND);
                self.current = self.current.advance_to(new_timestamp, step);
            }
            _ => {}
        }
    }

    pub fn adjust_speed(&mut self, new_speed: Choice<chrono::Duration>) {
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
            ControlEvent::Faster => self.adjust_speed(self.speed.next()),
            ControlEvent::Slower => self.adjust_speed(self.speed.prev()),
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
struct Seconds(f64);

impl Seconds {
    fn at_least(self, other: Self) -> Self {
        return Seconds(self.0.max(other.0));
    }
    fn at_most(self, other: Self) -> Self {
        return Seconds(self.0.min(other.0));
    }

    fn to_duration(self) -> chrono::Duration {
        chrono::Duration::nanoseconds((self.0 * 1e9) as i64)
    }
}

impl From<chrono::Duration> for Seconds {
    fn from(duration: chrono::Duration) -> Self {
        Seconds(duration.num_nanoseconds().unwrap() as f64 * 1e-9)
    }
}

impl From<std::time::Duration> for Seconds {
    fn from(duration: std::time::Duration) -> Self {
        Seconds(duration.as_secs_f64())
    }
}

impl Mul<f64> for Seconds {
    type Output = Seconds;
    fn mul(self, other: f64) -> Seconds {
        Seconds(self.0 * other)
    }
}

impl Div<f64> for Seconds {
    type Output = Seconds;
    fn div(self, other: f64) -> Seconds {
        Seconds(self.0 / other)
    }
}
