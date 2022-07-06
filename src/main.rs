use crate::render::Renderer;
use control::ControlEvent;
use kiss3d::window::Window;

mod body;
mod control;
mod render;
mod simulate;

fn main() {
    let mut window = Window::new_with_size("planets-rs", 1200, 800);
    let mut r = Renderer::new(&mut window);

    while r.frame(&mut window) {
        for mut event in window.events().iter() {
            if let Some(ev) = ControlEvent::from_window_event(&mut event) {
                r.handle_event(ev);
            }
        }
    }
}
