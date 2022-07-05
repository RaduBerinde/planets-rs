use crate::render::Renderer;
use kiss3d::window::Window;

mod body;
mod render;
mod simulate;

fn main() {
    let mut window = Window::new_with_size("planets-rs", 1200, 800);
    let mut r = Renderer::new(&mut window);

    while r.frame(&mut window) {}
}
