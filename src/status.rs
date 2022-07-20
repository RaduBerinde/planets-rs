use chrono::{DateTime, Utc};

use crate::{body::Body, choice::Choice};

// Status encapsulates the state that is presented in the UI.
pub struct Status {
    pub sim: SimulationStatus,
    pub render: RenderStatus,
}

pub struct SimulationStatus {
    pub timestamp: DateTime<Utc>,
    pub running: bool,
    pub speed: Choice<chrono::Duration>,
    pub reverse: bool,
}

pub struct RenderStatus {
    pub camera_focus: Choice<Body>,
}

impl Status {
    pub fn get(
        sim_provider: impl StatusProvider<SimulationStatus>,
        render_provider: impl StatusProvider<RenderStatus>,
    ) -> Self {
        Self {
            sim: sim_provider.status(),
            render: render_provider.status(),
        }
    }
}

pub trait StatusProvider<T> {
    fn status(&self) -> T;
}
