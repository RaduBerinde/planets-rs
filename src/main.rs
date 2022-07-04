use crate::{render::Renderer, system::*};
use kiss3d::window::Window;

mod body;
mod render;
mod system;

fn main() {
    let mut s = System::new();

    const APHELION: f64 = 152.10e6;
    s.earth.position.x = APHELION;

    // Distance between earth and moon during Aug 21, 2017 solar eclipse.
    const MOON_TO_EARTH: f64 = 372000.0;
    s.moon.position.x = s.earth.position.x - MOON_TO_EARTH;

    let mut window = Window::new_with_size("planets-rs", 1200, 800);
    let mut r = Renderer::new(&mut s, &mut window);

    while r.frame(&mut window) {}
}
