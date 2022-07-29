use kiss3d::event::{Action, Event, Key, WindowEvent};

use crate::choice::Choice;
use crate::config::{CameraSpec, Preset};
use crate::state::SimulationState;

#[derive(Clone)]
pub enum ControlEvent {
    CycleCamera,
    SetCamera(Choice<CameraSpec>),
    StartStop,
    JumpForward,
    JumpBack,
    Faster,
    Slower,
    SetSpeed(Choice<chrono::Duration>),
    Reverse,
    LoadPreset(Choice<Preset>),
    ToggleTrails,
    ToggleEcliptic,
    ToggleSkybox,
    Exit,
}

// Keyboard shortcut mappings. THe help message shows the mappings in this
// order.
const KEY_MAP: [(Key, ControlEvent); 12] = [
    (Key::Tab, ControlEvent::CycleCamera),
    (Key::Space, ControlEvent::StartStop),
    (Key::Equals, ControlEvent::Faster),
    (Key::Minus, ControlEvent::Slower),
    (Key::R, ControlEvent::Reverse),
    (Key::Left, ControlEvent::JumpBack),
    (Key::Right, ControlEvent::JumpForward),
    (Key::Escape, ControlEvent::Exit),
    (Key::Q, ControlEvent::Exit),
    (Key::T, ControlEvent::ToggleTrails),
    (Key::G, ControlEvent::ToggleEcliptic),
    (Key::S, ControlEvent::ToggleSkybox),
];

impl ControlEvent {
    pub fn from_window_event(
        event: &mut Event,
        sim_state: &dyn SimulationState,
    ) -> Option<ControlEvent> {
        #[allow(clippy::single_match)]
        match event.value {
            WindowEvent::Key(key, Action::Press, _) => {
                if let Some(mapping) = KEY_MAP.iter().find(|m| m.0 == key) {
                    event.inhibited = true;
                    return Some(mapping.1.clone());
                }
                if key >= Key::Key1 && key <= Key::Key9 {
                    let idx = key as usize - Key::Key1 as usize;
                    let presets = sim_state.preset().choice_set();
                    if idx < presets.len() {
                        event.inhibited = true;
                        return Some(ControlEvent::LoadPreset(presets.by_index(idx)));
                    }
                }
            }

            _ => (),
        }
        None
    }

    // pub fn description(&self) -> &'static str {
    //     match self {
    //         ControlEvent::CycleCamera => "Cycle camera focus",
    //         ControlEvent::StartStop => "Start/stop simulation",
    //         ControlEvent::Faster => "Increase the simulation speed",
    //         ControlEvent::Slower => "Decrease the simulation speed",
    //         ControlEvent::Reverse => "Reverse simulation",
    //     }
    // }
}
