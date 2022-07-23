use std::collections::HashMap;

use kiss3d::event::{Action, Event, Key, WindowEvent};

use crate::{body::Body, choice::Choice};

#[derive(Clone, PartialEq, Eq)]
pub enum ControlEvent {
    CycleCamera,
    SetCamera(Choice<Body>),
    StartStop,
    Faster,
    Slower,
    SetSpeed(Choice<chrono::Duration>),
    Reverse,
    Exit,
}

thread_local! {
    static KEY_MAP: HashMap<Key, ControlEvent> = HashMap::from([
        (Key::Tab, ControlEvent::CycleCamera),
        (Key::Space, ControlEvent::StartStop),
        (Key::Equals, ControlEvent::Faster),
        (Key::Minus, ControlEvent::Slower),
        (Key::R, ControlEvent::Reverse),
        (Key::Escape, ControlEvent::Exit),
    ]);
}

impl ControlEvent {
    pub fn from_window_event(event: &mut Event) -> Option<ControlEvent> {
        match event.value {
            WindowEvent::Key(key, Action::Press, _) => {
                let result = KEY_MAP.with(|km| km.get(&key).map(|ev| ev.clone()));
                if let Some(control_event) = result {
                    event.inhibited = true;
                    return Some(control_event.clone());
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
