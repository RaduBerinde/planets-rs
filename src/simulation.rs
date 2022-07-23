use std::{
    ops::{Deref, DerefMut},
    time::Instant,
};

use crate::{choice::ChoiceSet, status::SimulationStatus};

use self::seconds::Seconds;

use super::{body::BodyProperties, choice::Choice, control::ControlEvent};
use chrono::{DateTime, Utc};
use kiss3d::nalgebra::{Point3, Vector3};

mod seconds;
mod snapshot;

pub use snapshot::Snapshot;

pub struct Simulation {
    pub current: Snapshot,
    // Simulated duration per elapsed second.
    pub speed: Choice<chrono::Duration>,
    pub reverse: bool,
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
            speed: ChoiceSet::new(speeds).by_index(2),
            state: State::Stopped,
            reverse: false,
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

    const DEFAULT_STEP: Seconds = Seconds(60.0);
    const MIN_STEPS_PER_WALL_SECOND: f64 = 100.0;
    const MAX_STEPS_PER_WALL_SECOND: f64 = 10000.0;

    // When this limit becomes effective, we are not able to keep up with the wall time.
    const MAX_STEPS_PER_FRAME: u32 = 1000;

    pub fn advance(&mut self) {
        if let State::Running(start_info) = &self.state {
            let simulation_speed_per_sec = Seconds::from(self.speed.get());

            // Wall time elapsed since start().
            let elapsed = Seconds::from(start_info.instant.elapsed());
            assert!(elapsed.0 >= 0.0);

            // Simulation time elapsed since start().
            let simulation_elapsed = simulation_speed_per_sec * elapsed.0;

            let simulation_advance = if !self.reverse {
                let target_timestamp = start_info.timestamp + simulation_elapsed.to_duration();
                Seconds::from(target_timestamp - self.current.timestamp)
            } else {
                let target_timestamp = start_info.timestamp - simulation_elapsed.to_duration();
                Seconds::from(self.current.timestamp - target_timestamp)
            };

            let target_step = Simulation::DEFAULT_STEP
                .at_least(simulation_speed_per_sec / Simulation::MAX_STEPS_PER_WALL_SECOND)
                .at_most(simulation_speed_per_sec / Simulation::MIN_STEPS_PER_WALL_SECOND);

            let num_steps = (simulation_advance / target_step).ceil() as u32;
            self.advance_by(simulation_advance, num_steps);
        }
    }

    fn advance_by(&mut self, simulation_advance: Seconds, num_steps: u32) {
        assert!(simulation_advance.0 >= 0.0);
        let num_steps = num_steps.min(Simulation::MAX_STEPS_PER_FRAME);
        let step = simulation_advance / num_steps as f64 * if self.reverse { -1.0 } else { 1.0 };
        for _ in 0..num_steps {
            self.step(step);
        }
    }

    fn step(&mut self, dt: Seconds) {
        let s = &self.current;
        let new_timestamp = s.timestamp + dt.to_duration();
        let dt = dt.0;

        // We use velocity Verlet integration:
        //   x(t+dt) = x(t) + v(t)dt + a(t)dt^2/2
        //   v(t+dt) = v(t) + ((a(t) + a(t+dt))dt/2

        let (earth_acc, moon_acc) = gacc_earth_and_moon(&s.earth_position, &s.moon_position);

        let new_earth_pos = s.earth_position + s.earth_velocity * dt + 0.5 * earth_acc * dt * dt;
        let new_moon_pos = s.moon_position + s.moon_velocity * dt + 0.5 * moon_acc * dt * dt;

        let (new_earth_acc, new_moon_acc) = gacc_earth_and_moon(&new_earth_pos, &new_moon_pos);
        let new_earth_vel = s.earth_velocity + 0.5 * (earth_acc + new_earth_acc) * dt;
        let new_moon_vel = s.moon_velocity + 0.5 * (moon_acc + new_moon_acc) * dt;

        self.current = Snapshot {
            timestamp: new_timestamp,
            earth_position: new_earth_pos,
            earth_velocity: new_earth_vel,
            moon_position: new_moon_pos,
            moon_velocity: new_moon_vel,
        }
    }

    pub fn adjust_speed(&mut self, new_speed: Choice<chrono::Duration>) {
        // We need to stop and restart because advance assumes the
        // speed is unchanged since start.
        self.stopped().speed = new_speed;
    }

    pub fn reverse(&mut self) {
        let mut s = self.stopped();
        s.reverse = !s.reverse;
    }

    pub fn handle_event(&mut self, ev: &ControlEvent) {
        match ev {
            ControlEvent::StartStop => self.toggle_start(),
            ControlEvent::SetSpeed(s) => self.adjust_speed(s.clone()),
            ControlEvent::Faster => self.adjust_speed(self.speed.next()),
            ControlEvent::Slower => self.adjust_speed(self.speed.prev()),
            ControlEvent::Reverse => self.reverse(),
            ControlEvent::JumpForward | ControlEvent::JumpBack => {
                if !self.is_running() {
                    let old_reverse = self.reverse;
                    self.reverse = matches!(ev, ControlEvent::JumpBack);
                    let simulation_speed_per_sec = Seconds::from(self.speed.get());
                    self.advance_by(simulation_speed_per_sec, Self::MAX_STEPS_PER_FRAME);
                    self.reverse = old_reverse;
                }
            }
            _ => {}
        }
    }

    fn is_running(&self) -> bool {
        matches!(self.state, State::Running(..))
    }

    // stopped is used to stop the simulation and later restart it (if it was
    // running).
    fn stopped(&mut self) -> StoppedRef {
        let was_running = self.is_running();
        if was_running {
            self.stop();
        }
        StoppedRef {
            sim: self,
            needs_restart: was_running,
        }
    }
    pub fn status(&self) -> SimulationStatus {
        SimulationStatus {
            timestamp: self.current.timestamp,
            running: self.is_running(),
            speed: self.speed.clone(),
            reverse: self.reverse,
        }
    }
}

// StoppedRef is used internally to temporarily stop the simulation to make changes.
struct StoppedRef<'a> {
    sim: &'a mut Simulation,
    needs_restart: bool,
}

impl<'a> Drop for StoppedRef<'a> {
    fn drop(&mut self) {
        if self.needs_restart {
            self.sim.start();
        }
    }
}

impl<'a> Deref for StoppedRef<'a> {
    type Target = Simulation;

    fn deref(&self) -> &Self::Target {
        self.sim
    }
}

impl<'a> DerefMut for StoppedRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.sim
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
    vec.normalize() * amount
}
