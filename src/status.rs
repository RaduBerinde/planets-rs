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
    pub show_trails: bool,
}
