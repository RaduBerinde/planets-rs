use chrono::{DateTime, Utc};

use crate::{
    choice::Choice,
    config::{CameraSpec, Preset},
};

// SimulationState is a trait used to inform the UI on the current state of the
// simulation.
pub trait SimulationState {
    fn preset(&self) -> Choice<Preset>;
    fn timestamp(&self) -> DateTime<Utc>;
    fn is_running(&self) -> bool;
    fn speed(&self) -> Choice<chrono::Duration>;
    fn is_reverse(&self) -> bool;
}

// RenderState is a trait used to inform the UI on the current state and
// settings of the renderer.
pub trait RenderState {
    fn camera_focus(&self) -> Choice<CameraSpec>;
    fn show_trails(&self) -> bool;
    fn show_ecliptic(&self) -> bool;
    fn show_skybox(&self) -> bool;
    fn show_earth_axis(&self) -> bool;
    fn fps(&self) -> f64;
}
