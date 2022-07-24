use crate::render::Renderer;
use control::ControlEvent;
use kiss3d::window::{CanvasSetup, NumSamples, Window};
use simulation::{Simulation, Snapshot};

mod body;
mod choice;
mod control;
mod render;
mod simulation;
mod state;

fn main() {
    let setup = CanvasSetup {
        vsync: true,
        samples: NumSamples::Zero,
    };
    let mut window = Window::new_with_setup("planets-rs", 1200, 800, setup);
    let mut sim = Simulation::new(Snapshot::simple());
    let mut r = Renderer::new(&sim.current, &mut window);

    loop {
        let events = r.frame(&mut window, &sim);

        for event in events {
            if matches!(event, ControlEvent::Exit) {
                return;
            }
            r.handle_event(&event);
            sim.handle_event(&event);
        }
        sim.advance();
        r.set_snapshot(&sim.current);
    }
}
