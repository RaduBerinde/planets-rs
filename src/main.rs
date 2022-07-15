use crate::render::Renderer;
use control::ControlEvent;
use kiss3d::window::Window;
use simulation::{Simulation, Snapshot};

mod body;
mod choice;
mod control;
mod render;
mod simulation;

fn main() {
    let mut window = Window::new_with_size("planets-rs", 1200, 800);
    let mut sim = Simulation::new(Snapshot::simple());
    let mut r = Renderer::new(&sim.current, &mut window);

    while r.frame(&mut window) {
        for mut event in window.events().iter() {
            if let Some(ev) = ControlEvent::from_window_event(&mut event) {
                r.handle_event(ev);
                sim.handle_event(ev);
            }
        }
        sim.advance();
        r.set_snapshot(&sim.current);
    }
}
