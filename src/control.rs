use enum_iterator::{all, Sequence};
use kiss3d::event::{Action, Event, Key, WindowEvent};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Sequence)]
pub enum ControlEvent {
    CycleCamera,
    StartStop,
    Faster,
    Slower,
}

impl ControlEvent {
    pub fn from_window_event(event: &mut Event) -> Option<ControlEvent> {
        match event.value {
            WindowEvent::Key(key, Action::Press, _) => {
                for ev in all::<ControlEvent>() {
                    if key == ev.key() {
                        event.inhibited = true;
                        return Some(ev);
                    }
                }
            }
            _ => (),
        }
        None
    }

    pub fn key(&self) -> Key {
        match self {
            ControlEvent::CycleCamera => Key::Tab,
            ControlEvent::StartStop => Key::Space,
            ControlEvent::Faster => Key::Equals,
            ControlEvent::Slower => Key::Minus,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ControlEvent::CycleCamera => "Cycle camera focus",
            ControlEvent::StartStop => "Start/stop simulation",
            ControlEvent::Faster => "Increase the simulation speed",
            ControlEvent::Slower => "Decrease the simulation speed",
        }
    }
}
