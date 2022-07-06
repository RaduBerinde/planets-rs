use enum_map::{enum_map, Enum, EnumMap};
use kiss3d::event::{Action, Event, Key, WindowEvent};
use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Enum)]
pub enum ControlEvent {
    CycleCamera,
    StartStop,
    Faster,
    Slower,
}

lazy_static! {
    static ref KEY_MAP: EnumMap<ControlEvent, Key> = enum_map! {
        ControlEvent::CycleCamera => Key::Tab,
        ControlEvent::StartStop => Key::Space,
        ControlEvent::Faster => Key::Equals,
        ControlEvent::Slower => Key::Minus,
    };
}

impl ControlEvent {
    pub fn from_window_event(event: &mut Event) -> Option<ControlEvent> {
        match event.value {
            WindowEvent::Key(key, Action::Press, _) => {
                for (ev, &ev_key) in KEY_MAP.iter() {
                    if key == ev_key {
                        event.inhibited = true;
                        return Some(ev);
                    }
                }
                //for ev in all::<ControlEvent>() {
                //    if key == ev.key() {
                //        event.inhibited = true;
                //        return Some(ev);
                //    }
                //}
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
        }
    }
}
