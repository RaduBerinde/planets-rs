use std::collections::HashMap;

use kiss3d::event::{Action, Event, Key, WindowEvent};

use crate::{choice::Choice, config::CameraSpec};

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
    ToggleTrails,
    ToggleEcliptic,
    ToggleSkybox,
    Exit,
}

thread_local! {
    static KEY_MAP: HashMap<Key, ControlEvent> = HashMap::from([
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
    ]);
}

impl ControlEvent {
    pub fn from_window_event(event: &mut Event) -> Option<ControlEvent> {
        #[allow(clippy::single_match)]
        match event.value {
            WindowEvent::Key(key, Action::Press, _) => {
                let result = KEY_MAP.with(|km| km.get(&key).cloned());
                if let Some(control_event) = result {
                    event.inhibited = true;
                    return Some(control_event);
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
