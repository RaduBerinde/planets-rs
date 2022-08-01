use config::Config;
use control::ControlEvent;
use kiss3d::{
    nalgebra::{Point2, Point3},
    text::Font,
    window::{CanvasSetup, NumSamples, Window},
};
use render::Renderer;
use simulation::Simulation;

mod body;
mod choice;
mod config;
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
    // Do an initial render before we initialize.
    window.draw_text(
        "Loading textures..",
        &Point2::new(200.0, 200.0),
        100.0,
        &Font::default(),
        &Point3::new(0.8, 0.8, 0.8),
    );
    window.render();
    let config = Config::default();
    let mut sim = Simulation::new(&config.initial_preset, &config.initial_speed);
    let mut r = Renderer::new(sim.current(), &mut window, &config.initial_camera);

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
        r.set_snapshot(sim.current());
    }
}
