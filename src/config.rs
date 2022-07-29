use crate::{
    body::Body,
    choice::{Choice, ChoiceSet},
    simulation::Snapshot,
};

pub struct Config {
    pub initial_preset: Choice<Preset>,
    pub initial_camera: Choice<CameraSpec>,
    pub initial_speed: Choice<chrono::Duration>,
}

#[derive(Clone, Copy)]
pub struct Preset {
    pub name: &'static str,
    pub snapshot: Snapshot,
}

#[derive(Clone, Copy)]
pub struct CameraSpec {
    pub focus: Body,
    pub direction: CameraDirection,
    // Distance from body, as a multiple of its radius.
    pub relative_dist: f64,
    pub description: &'static str,
}

#[derive(Clone, Copy)]
pub enum CameraDirection {
    FromAbove,
    FromBody(Body),
}

impl Config {
    pub fn default() -> Self {
        let presets = [
            Preset {
                name: "test - no moon inclination",
                snapshot: Snapshot::test1(),
            },
            Preset {
                name: "test - high moon inclination",
                snapshot: Snapshot::test2(),
            },
        ];
        let initial_preset = ChoiceSet::new(presets).by_index(0);

        let camera_specs = [
            CameraSpec {
                focus: Body::Earth,
                direction: CameraDirection::FromAbove,
                relative_dist: 10.0,
                description: "Earth",
            },
            CameraSpec {
                focus: Body::Moon,
                direction: CameraDirection::FromAbove,
                relative_dist: 30.0,
                description: "Moon",
            },
            CameraSpec {
                focus: Body::Moon,
                direction: CameraDirection::FromBody(Body::Earth),
                relative_dist: 10.0,
                description: "Moon phase",
            },
            CameraSpec {
                focus: Body::Sun,
                direction: CameraDirection::FromAbove,
                relative_dist: 100.0,
                description: "Sun",
            },
        ];
        let initial_camera = ChoiceSet::new(camera_specs).by_index(0);

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
        let initial_speed = ChoiceSet::new(speeds).by_index(2);
        Self {
            initial_preset,
            initial_camera,
            initial_speed,
        }
    }
}
