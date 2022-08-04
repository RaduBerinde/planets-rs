use std::fmt::Write;

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
    ToggleEarthAxis,
    ToggleHelp,
    Exit,
}

// Keyboard shortcut mappings. THe help message shows the mappings in this
// order.
const KEY_MAP: [(Key, ControlEvent); 14] = [
    (Key::Space, ControlEvent::StartStop),
    (Key::Tab, ControlEvent::CycleCamera),
    (Key::Equals, ControlEvent::Faster),
    (Key::Minus, ControlEvent::Slower),
    (Key::R, ControlEvent::Reverse),
    (Key::Left, ControlEvent::JumpBack),
    (Key::Right, ControlEvent::JumpForward),
    (Key::T, ControlEvent::ToggleTrails),
    (Key::G, ControlEvent::ToggleEcliptic),
    (Key::S, ControlEvent::ToggleSkybox),
    (Key::X, ControlEvent::ToggleEarthAxis),
    (Key::H, ControlEvent::ToggleHelp),
    (Key::Escape, ControlEvent::Exit),
    (Key::Q, ControlEvent::Exit),
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

    pub fn description(&self) -> &'static str {
        match self {
            ControlEvent::CycleCamera => "Cycle camera focus",
            ControlEvent::StartStop => "Start/stop simulation",
            ControlEvent::Faster => "Increase the simulation speed",
            ControlEvent::Slower => "Decrease the simulation speed",
            ControlEvent::Reverse => "Reverse simulation",
            ControlEvent::SetCamera(_) => "Set camera focus",
            ControlEvent::JumpForward => "Jump forward",
            ControlEvent::JumpBack => "Jump backward",
            ControlEvent::SetSpeed(_) => "Set simulation speed",
            ControlEvent::LoadPreset(_) => "Load simulation preset",
            ControlEvent::ToggleTrails => "Toggle rendering of trails",
            ControlEvent::ToggleEcliptic => "Toggle rendering of orbital plane",
            ControlEvent::ToggleSkybox => "Toggle sky background",
            ControlEvent::ToggleEarthAxis => "Toggle earth axis",
            ControlEvent::ToggleHelp => "Toggle help",
            ControlEvent::Exit => "Exit",
        }
    }
}

pub fn help_text() -> String {
    let mut entries: Vec<(String, &'static str)> = vec![
        ("Mouse scroll".to_string(), "Zoom camera"),
        ("Click + drag".to_string(), "Rotate camera"),
    ];
    let key_str = |k: &Key| -> String {
        match k {
            Key::Equals => "=".to_string(),
            Key::Minus => "-".to_string(),
            _ => format!("{:?}", k),
        }
    };
    for (k, e) in &KEY_MAP {
        if !matches!(e, ControlEvent::Exit) {
            entries.push((key_str(k), e.description()));
        }
    }
    entries.push(("1, 2".to_string(), "Load preset"));
    entries.push(("Q, Esc".to_string(), ControlEvent::Exit.description()));
    let width = entries.iter().map(|e| e.0.len()).max().unwrap();

    let mut s = String::new();
    for e in entries {
        let _ = writeln!(&mut s, "{:>width$}  {}", e.0, e.1, width = width);
    }
    s
}
